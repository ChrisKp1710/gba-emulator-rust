/// PPU Mode 4 - Bitmap Paletted (240x160, 8-bit indexed)
///
/// Mode 4 uses 8-bit indexed color with 256-color palette.
/// Two frame buffers for page flipping (double buffering).
/// Frame 0: 0x06000000-0x06009600 (38400 bytes)
/// Frame 1: 0x0600A000-0x06013600 (38400 bytes)
use super::constants::*;

/// Render Mode 4 scanline - 8-bit paletted bitmap
pub fn render_mode4_scanline(
    framebuffer: &mut [u16],
    vram: &[u8],
    palette_ram: &[u8],
    scanline: usize,
    frame_select: bool,
) {
    // Page flip: frame 0 or frame 1
    let frame_offset = if frame_select { 0xA000 } else { 0x0000 };

    let line_offset = scanline * SCREEN_WIDTH;

    for x in 0..SCREEN_WIDTH {
        let vram_addr = frame_offset + (scanline * SCREEN_WIDTH + x);

        // Read 8-bit palette index from VRAM
        let palette_index = vram.get(vram_addr).copied().unwrap_or(0) as usize;

        // Lookup RGB555 color in BG palette (first 512 bytes)
        let color_addr = palette_index * 2;
        if color_addr + 1 < BG_PALETTE_SIZE {
            let color_low = palette_ram[color_addr] as u16;
            let color_high = palette_ram[color_addr + 1] as u16;
            let rgb555 = color_low | (color_high << 8);
            framebuffer[line_offset + x] = rgb555;
        } else {
            framebuffer[line_offset + x] = 0; // Black for invalid index
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode4_basic_render() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000]; // 96KB VRAM
        let mut palette_ram = vec![0u8; PALETTE_RAM_SIZE];

        // Setup palette: index 1 = red (0x001F), index 2 = green (0x03E0)
        palette_ram[2] = 0x1F; // Red low byte
        palette_ram[3] = 0x00; // Red high byte
        palette_ram[4] = 0xE0; // Green low byte
        palette_ram[5] = 0x03; // Green high byte

        // Draw some pixels on scanline 0 (frame 0)
        vram[0] = 1; // Pixel 0 = red
        vram[1] = 2; // Pixel 1 = green
        vram[100] = 1; // Pixel 100 = red

        render_mode4_scanline(&mut framebuffer, &vram, &palette_ram, 0, false);

        assert_eq!(framebuffer[0], 0x001F); // Red
        assert_eq!(framebuffer[1], 0x03E0); // Green
        assert_eq!(framebuffer[100], 0x001F); // Red
        assert_eq!(framebuffer[2], 0); // Black (index 0)
    }

    #[test]
    fn test_mode4_page_flip() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000];
        let mut palette_ram = vec![0u8; PALETTE_RAM_SIZE];

        // Setup blue color (index 1)
        palette_ram[2] = 0x00;
        palette_ram[3] = 0x7C; // 0x7C00 = blue

        // Frame 0: pixel at (0,0)
        vram[0] = 1;

        // Frame 1: pixel at (0,0) - offset 0xA000
        vram[0xA000] = 1;

        // Render frame 0
        render_mode4_scanline(&mut framebuffer, &vram, &palette_ram, 0, false);
        assert_eq!(framebuffer[0], 0x7C00); // Blue from frame 0

        // Clear and render frame 1
        framebuffer.fill(0);
        render_mode4_scanline(&mut framebuffer, &vram, &palette_ram, 0, true);
        assert_eq!(framebuffer[0], 0x7C00); // Blue from frame 1
    }

    #[test]
    fn test_mode4_256_colors() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000];
        let mut palette_ram = vec![0u8; PALETTE_RAM_SIZE];

        // Setup 256-color palette (gradient)
        for i in 0..256 {
            let color = (i as u16) | ((i as u16) << 5) | ((i as u16) << 10);
            palette_ram[i * 2] = (color & 0xFF) as u8;
            palette_ram[i * 2 + 1] = ((color >> 8) & 0xFF) as u8;
        }

        // Draw gradient on scanline 0
        for x in 0..SCREEN_WIDTH {
            vram[x] = (x % 256) as u8;
        }

        render_mode4_scanline(&mut framebuffer, &vram, &palette_ram, 0, false);

        // Verify gradient
        for x in 0..SCREEN_WIDTH {
            let expected_index = (x % 256) as u16;
            let expected_color = expected_index | (expected_index << 5) | (expected_index << 10);
            assert_eq!(framebuffer[x], expected_color);
        }
    }

    #[test]
    fn test_mode4_scanline_offset() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000];
        let mut palette_ram = vec![0u8; PALETTE_RAM_SIZE];

        // White color (index 255)
        palette_ram[510] = 0xFF;
        palette_ram[511] = 0x7F; // 0x7FFF = white

        // Pixel at scanline 50, x=100
        vram[50 * SCREEN_WIDTH + 100] = 255;

        render_mode4_scanline(&mut framebuffer, &vram, &palette_ram, 50, false);

        assert_eq!(framebuffer[50 * SCREEN_WIDTH + 100], 0x7FFF); // White
    }
}
