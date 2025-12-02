use super::constants::*;
use super::types::BgControl;

/// Render scanline in Mode 0 (4 tiled backgrounds)
#[allow(clippy::too_many_arguments)]
pub fn render_mode0_scanline(
    scanline: usize,
    screen_width: usize,
    dispcnt: u16,
    bg_control: &[BgControl; 4],
    bg_hofs: &[u16; 4],
    bg_vofs: &[u16; 4],
    vram: &[u8],
    palette_ram: &[u8],
    framebuffer: &mut [u16],
) {
    // Temporary buffer for pixels of each layer with priority
    // (color_rgb555, priority, has_pixel)
    let mut layers: [Vec<(u16, u8, bool)>; 4] = [
        vec![(0, 0, false); screen_width],
        vec![(0, 0, false); screen_width],
        vec![(0, 0, false); screen_width],
        vec![(0, 0, false); screen_width],
    ];

    // Render each background if enabled
    for (bg_num, layer) in layers.iter_mut().enumerate() {
        // Check if BG is enabled in DISPCNT
        if (dispcnt & (1 << (8 + bg_num))) == 0 {
            continue;
        }

        render_bg_scanline(
            vram,
            palette_ram,
            bg_num,
            &bg_control[bg_num],
            bg_hofs[bg_num],
            bg_vofs[bg_num],
            layer,
            scanline,
            screen_width,
        );
    }

    // Compositing: lower priority = in front
    // For each pixel X, find the layer with lowest priority that has a pixel
    for x in 0..screen_width {
        let mut final_color = 0u16; // Backdrop (black)
        let mut found = false;

        // Scan all priorities from 0 to 3
        for priority in 0..=3 {
            // Check each layer for this priority
            for layer in &layers {
                let (color, layer_priority, has_pixel) = layer[x];
                if has_pixel && layer_priority == priority {
                    final_color = color;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        framebuffer[scanline * screen_width + x] = final_color;
    }
}

/// Render a single background for a scanline
#[allow(clippy::too_many_arguments)]
fn render_bg_scanline(
    vram: &[u8],
    palette_ram: &[u8],
    _bg_num: usize,
    bg_control: &BgControl,
    scroll_x: u16,
    scroll_y: u16,
    layer: &mut [(u16, u8, bool)],
    line: usize,
    screen_width: usize,
) {
    let priority = bg_control.priority;

    // Calculate Y position with scrolling
    let scroll_y = scroll_y as usize;
    let y = (line + scroll_y) & 0x1FF; // Wrap to 512 pixel max

    let (screen_width_tiles, screen_height_tiles) = bg_control.get_screen_size();
    let tile_y = y / 8;

    // If out of tilemap bounds, skip
    if tile_y >= screen_height_tiles {
        return;
    }

    let scroll_x = scroll_x as usize;

    // Render 240 pixels + extra for partial tiles
    for (x, pixel) in layer.iter_mut().enumerate().take(screen_width) {
        let pixel_x = (x + scroll_x) & 0x1FF; // Wrap to 512 pixel max
        let tile_x = pixel_x / 8;

        if tile_x >= screen_width_tiles {
            continue;
        }

        // Read tile entry from tilemap
        let screen_base_addr = (bg_control.screen_base as usize) * 2048;
        let tile_offset = tile_y * screen_width_tiles + tile_x;
        let tile_entry_addr = screen_base_addr + tile_offset * 2;

        if tile_entry_addr + 1 >= vram.len() {
            continue;
        }

        // Tile entry: 16-bit
        // Bits 0-9: Tile number
        // Bit 10: H-flip
        // Bit 11: V-flip
        // Bits 12-15: Palette number (16-color mode only)
        let tile_entry =
            (vram[tile_entry_addr] as u16) | ((vram[tile_entry_addr + 1] as u16) << 8);
        let tile_num = (tile_entry & 0x3FF) as usize;
        let h_flip = (tile_entry & (1 << 10)) != 0;
        let v_flip = (tile_entry & (1 << 11)) != 0;
        let palette_bank = ((tile_entry >> 12) & 0xF) as usize;

        // Calculate pixel position within tile (0-7)
        let mut tile_pixel_x = pixel_x % 8;
        let mut tile_pixel_y = y % 8;

        if h_flip {
            tile_pixel_x = 7 - tile_pixel_x;
        }
        if v_flip {
            tile_pixel_y = 7 - tile_pixel_y;
        }

        // Read pixel from tile data
        let char_base_addr = (bg_control.char_base as usize) * 16384;
        let palette_index = if bg_control.palette_256 {
            // 256 colors: 1 byte per pixel, 64 bytes per tile
            let tile_addr = char_base_addr + tile_num * 64;
            let pixel_addr = tile_addr + tile_pixel_y * 8 + tile_pixel_x;
            if pixel_addr >= vram.len() {
                0
            } else {
                vram[pixel_addr] as usize
            }
        } else {
            // 16 colors: 4 bits per pixel, 32 bytes per tile
            let tile_addr = char_base_addr + tile_num * 32;
            let pixel_addr = tile_addr + tile_pixel_y * 4 + tile_pixel_x / 2;
            if pixel_addr >= vram.len() {
                0
            } else {
                let byte = vram[pixel_addr];
                if tile_pixel_x & 1 == 0 {
                    (byte & 0xF) as usize
                } else {
                    ((byte >> 4) & 0xF) as usize
                }
            }
        };

        // Color 0 = transparent
        if palette_index == 0 {
            continue;
        }

        // Lookup color in palette
        let color = if bg_control.palette_256 {
            // 256 color palette
            read_bg_palette(palette_ram, palette_index)
        } else {
            // 16x16 palette
            let palette_offset = palette_bank * 16 + palette_index;
            read_bg_palette(palette_ram, palette_offset)
        };

        *pixel = (color, priority, true);
    }
}

/// Read RGB555 color from BG palette RAM
fn read_bg_palette(palette_ram: &[u8], index: usize) -> u16 {
    let addr = index * 2;
    if addr + 1 < BG_PALETTE_SIZE {
        (palette_ram[addr] as u16) | ((palette_ram[addr + 1] as u16) << 8)
    } else {
        0
    }
}
