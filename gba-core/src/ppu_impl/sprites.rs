use super::constants::*;

/// Sprite Attribute (OAM entry)
#[derive(Debug, Clone, Copy)]
pub struct SpriteAttribute {
    // Attribute 0 (16-bit)
    pub y: u8,             // Bits 0-7: Y coordinate
    pub obj_mode: u8,      // Bits 8-9: Object mode (normal, affine, disabled, double)
    pub gfx_mode: u8,      // Bits 10-11: GFX mode (normal, alpha, window)
    pub mosaic: bool,      // Bit 12: Mosaic
    pub palette_256: bool, // Bit 13: 256 colors (true) or 16 colors (false)
    pub shape: u8,         // Bits 14-15: Shape (square, wide, tall)

    // Attribute 1 (16-bit)
    pub x: u16,       // Bits 0-8: X coordinate (9 bits)
    pub h_flip: bool, // Bit 12: Horizontal flip (regular sprites)
    pub v_flip: bool, // Bit 13: Vertical flip (regular sprites)
    pub size: u8,     // Bits 14-15: Size

    // Attribute 2 (16-bit)
    pub tile_index: u16,  // Bits 0-9: Tile number
    pub priority: u8,     // Bits 10-11: Priority
    pub palette_bank: u8, // Bits 12-15: Palette bank (16-color mode)
}

impl SpriteAttribute {
    /// Create sprite from 6 OAM bytes (first 6 bytes, last 2 are rotation/scaling)
    pub fn from_oam_bytes(bytes: &[u8]) -> Self {
        if bytes.len() < 6 {
            return Self::default();
        }

        let attr0 = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        let attr1 = (bytes[2] as u16) | ((bytes[3] as u16) << 8);
        let attr2 = (bytes[4] as u16) | ((bytes[5] as u16) << 8);

        Self {
            // Attr 0
            y: (attr0 & 0xFF) as u8,
            obj_mode: ((attr0 >> 8) & 0x3) as u8,
            gfx_mode: ((attr0 >> 10) & 0x3) as u8,
            mosaic: (attr0 & (1 << 12)) != 0,
            palette_256: (attr0 & (1 << 13)) != 0,
            shape: ((attr0 >> 14) & 0x3) as u8,

            // Attr 1
            x: attr1 & 0x1FF,
            h_flip: (attr1 & (1 << 12)) != 0,
            v_flip: (attr1 & (1 << 13)) != 0,
            size: ((attr1 >> 14) & 0x3) as u8,

            // Attr 2
            tile_index: attr2 & 0x3FF,
            priority: ((attr2 >> 10) & 0x3) as u8,
            palette_bank: ((attr2 >> 12) & 0xF) as u8,
        }
    }

    /// Get sprite dimensions in pixels (width, height)
    pub fn get_size(&self) -> (usize, usize) {
        match (self.shape, self.size) {
            // Square
            (0, 0) => (8, 8),
            (0, 1) => (16, 16),
            (0, 2) => (32, 32),
            (0, 3) => (64, 64),
            // Wide (horizontal)
            (1, 0) => (16, 8),
            (1, 1) => (32, 8),
            (1, 2) => (32, 16),
            (1, 3) => (64, 32),
            // Tall (vertical)
            (2, 0) => (8, 16),
            (2, 1) => (8, 32),
            (2, 2) => (16, 32),
            (2, 3) => (32, 64),
            _ => (8, 8),
        }
    }

    /// Check if sprite is visible
    pub fn is_visible(&self) -> bool {
        // obj_mode == 2 means disabled
        self.obj_mode != 2
    }
}

impl Default for SpriteAttribute {
    fn default() -> Self {
        Self {
            y: 0,
            obj_mode: 2, // Disabled
            gfx_mode: 0,
            mosaic: false,
            palette_256: false,
            shape: 0,
            x: 0,
            h_flip: false,
            v_flip: false,
            size: 0,
            tile_index: 0,
            priority: 0,
            palette_bank: 0,
        }
    }
}

/// Render sprites for current scanline
pub fn render_sprites_scanline(
    scanline: usize,
    screen_width: usize,
    oam: &[u8],
    vram: &[u8],
    palette_ram: &[u8],
    framebuffer: &mut [u16],
) {
    // Sprite priority buffer (color, priority, has_sprite)
    let mut sprite_buffer: Vec<(u16, u8, bool)> = vec![(0, 4, false); screen_width];

    // Render sprites in reverse order (higher index = behind)
    for sprite_idx in (0..OAM_SPRITE_COUNT).rev() {
        let offset = sprite_idx * 8;
        if offset + 6 > oam.len() {
            continue;
        }
        let sprite = SpriteAttribute::from_oam_bytes(&oam[offset..offset + 6]);

        if !sprite.is_visible() {
            continue;
        }

        let (sprite_width, sprite_height) = sprite.get_size();
        let sprite_y = sprite.y as usize;

        // Check if sprite intersects this scanline
        let y_in_sprite = if scanline >= sprite_y {
            scanline.wrapping_sub(sprite_y)
        } else {
            // Wrap-around for Y > 160
            scanline.wrapping_add(256).wrapping_sub(sprite_y)
        };

        if y_in_sprite >= sprite_height {
            continue;
        }

        // Apply V-flip
        let actual_y = if sprite.v_flip {
            sprite_height - 1 - y_in_sprite
        } else {
            y_in_sprite
        };

        // Render each sprite pixel
        for sprite_x in 0..sprite_width {
            let screen_x = (sprite.x as usize).wrapping_add(sprite_x) & 0x1FF;

            if screen_x >= screen_width {
                continue;
            }

            // Apply H-flip
            let actual_x = if sprite.h_flip {
                sprite_width - 1 - sprite_x
            } else {
                sprite_x
            };

            // Calculate tile and pixel within tile
            let tiles_per_row = sprite_width / 8;
            let tile_x = actual_x / 8;
            let tile_y = actual_y / 8;
            let pixel_x = actual_x % 8;
            let pixel_y = actual_y % 8;

            // Calculate tile index
            let tile_offset = if sprite.palette_256 {
                // 256 colors: sequential tiles
                tile_y * tiles_per_row + tile_x
            } else {
                // 16 colors: 2D tile layout
                tile_y * 32 + tile_x
            };

            let tile_num = sprite.tile_index as usize + tile_offset;

            // Read pixel from tile in VRAM OBJ
            let palette_index = if sprite.palette_256 {
                // 256 colors: 64 bytes per tile
                let tile_addr = OBJ_TILE_BASE + tile_num * 64;
                let pixel_addr = tile_addr + pixel_y * 8 + pixel_x;
                if pixel_addr < vram.len() {
                    vram[pixel_addr] as usize
                } else {
                    0
                }
            } else {
                // 16 colors: 32 bytes per tile
                let tile_addr = OBJ_TILE_BASE + tile_num * 32;
                let pixel_addr = tile_addr + pixel_y * 4 + pixel_x / 2;
                if pixel_addr < vram.len() {
                    let byte = vram[pixel_addr];
                    if pixel_x & 1 == 0 {
                        (byte & 0xF) as usize
                    } else {
                        ((byte >> 4) & 0xF) as usize
                    }
                } else {
                    0
                }
            };

            // Color 0 = transparent
            if palette_index == 0 {
                continue;
            }

            // Lookup in OBJ palette
            let color = if sprite.palette_256 {
                // 256 colors
                read_obj_palette(palette_ram, palette_index)
            } else {
                // 16 colors
                let palette_offset = sprite.palette_bank as usize * 16 + palette_index;
                read_obj_palette(palette_ram, palette_offset)
            };

            // Check priority: sprite with lower priority (number) = in front
            let (_, current_priority, has_sprite) = sprite_buffer[screen_x];
            if !has_sprite || sprite.priority <= current_priority {
                sprite_buffer[screen_x] = (color, sprite.priority, true);
            }
        }
    }

    // Composite sprites onto framebuffer
    for (x, &(sprite_color, _sprite_priority, has_sprite)) in sprite_buffer.iter().enumerate() {
        if has_sprite {
            // TODO: Consider BG vs OBJ priority
            // For now sprites always on top of background
            framebuffer[scanline * screen_width + x] = sprite_color;
        }
    }
}

/// Read RGB555 color from OBJ palette
fn read_obj_palette(palette_ram: &[u8], index: usize) -> u16 {
    let addr = OBJ_PALETTE_OFFSET + index * 2;
    if addr + 1 < PALETTE_RAM_SIZE {
        (palette_ram[addr] as u16) | ((palette_ram[addr + 1] as u16) << 8)
    } else {
        0
    }
}
