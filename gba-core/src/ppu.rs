/// PPU - Picture Processing Unit
/// Modular implementation in ppu_impl/
pub use crate::ppu_impl::{
    BgControl,
    DisplayMode,
    SpriteAttribute,
    // Constants
    BG0CNT,
    BG0HOFS,
    BG0VOFS,
    BG1CNT,
    BG1HOFS,
    BG1VOFS,
    BG2CNT,
    BG2HOFS,
    BG2VOFS,
    BG3CNT,
    BG3HOFS,
    BG3VOFS,
    DISPCNT,
    DISPSTAT,
    PPU,
    SCREEN_HEIGHT,
    SCREEN_WIDTH,
    VCOUNT,
};

#[cfg(test)]
mod tests {
    use crate::ppu_impl;

    use super::*;

    const SCREEN_WIDTH: usize = ppu_impl::SCREEN_WIDTH;

    #[test]
    fn test_bg_control_parsing() {
        let bg_ctrl = BgControl {
            priority: 2,
            char_base: 1,
            mosaic: false,
            palette_256: true,
            screen_base: 10,
            wrap: false,
            screen_size: 1,
        };

        let value = bg_ctrl.to_u16();
        let parsed = BgControl::from_u16(value);

        assert_eq!(parsed.priority, 2);
        assert_eq!(parsed.char_base, 1);
        assert!(parsed.palette_256);
        assert_eq!(parsed.screen_base, 10);
        assert_eq!(parsed.screen_size, 1);
    }

    #[test]
    fn test_bg_screen_size() {
        let configs = [(0, (32, 32)), (1, (64, 32)), (2, (32, 64)), (3, (64, 64))];

        for (screen_size, expected) in configs {
            let bg_ctrl = BgControl {
                screen_size,
                ..BgControl::default()
            };
            assert_eq!(bg_ctrl.get_screen_size(), expected);
        }
    }

    #[test]
    fn test_palette_ram_access() {
        let mut ppu = PPU::new();

        ppu.write_palette_halfword(0, 0x7FFF);
        assert_eq!(ppu.read_palette_halfword(0), 0x7FFF);

        ppu.write_palette_halfword(10, 0x001F);
        assert_eq!(ppu.read_palette_halfword(10), 0x001F);

        ppu.write_palette_byte(20, 0xAB);
        ppu.write_palette_byte(21, 0xCD);
        assert_eq!(ppu.read_palette_halfword(20), 0xCDAB);
    }

    #[test]
    fn test_mode0_simple_tile() {
        let mut ppu = PPU::new();
        ppu.dispcnt = 0x0100;

        ppu.bg_control[0] = BgControl {
            priority: 0,
            char_base: 0,
            mosaic: false,
            palette_256: false,
            screen_base: 8,
            wrap: false,
            screen_size: 0,
        };

        let mut vram = vec![0u8; 96 * 1024];

        ppu.palette_ram[0] = 0x00;
        ppu.palette_ram[1] = 0x00;
        ppu.palette_ram[2] = 0x1F;
        ppu.palette_ram[3] = 0x00;

        vram.iter_mut().take(32).for_each(|v| *v = 0x11);

        let tilemap_offset = 16384;
        vram[tilemap_offset] = 0x00;
        vram[tilemap_offset + 1] = 0x00;

        ppu.scanline = 0;
        ppu.step(1232, &vram);

        for x in 0..8 {
            assert_eq!(ppu.framebuffer[x], 0x001F, "Pixel {} should be red", x);
        }
    }

    #[test]
    fn test_mode0_scrolling() {
        let mut ppu = PPU::new();
        ppu.write_register(DISPCNT, 0x0100);
        ppu.write_register(BG0CNT, 0x0000);
        ppu.write_register(BG0HOFS, 8);
        ppu.write_register(BG0VOFS, 0);

        let mut vram = vec![0u8; 96 * 1024];
        ppu.write_palette_halfword(2, 0x7C00);

        vram.iter_mut().take(32).for_each(|v| *v = 0x11);
        vram.iter_mut().skip(32).take(32).for_each(|v| *v = 0x00);

        vram[0] = 0x00;
        vram[1] = 0x00;
        vram[2] = 0x01;
        vram[3] = 0x00;

        ppu.scanline = 0;
        ppu.step(1232, &vram);

        assert_eq!(ppu.framebuffer[0], 0x0000);
    }

    #[test]
    fn test_mode0_priority() {
        let mut ppu = PPU::new();
        ppu.write_register(DISPCNT, 0x0300);
        ppu.write_register(BG0CNT, 0x0001);
        ppu.write_register(BG1CNT, 0x0000);

        let mut vram = vec![0u8; 96 * 1024];
        ppu.write_palette_halfword(2, 0x001F);
        ppu.write_palette_halfword(4, 0x03E0);

        vram.iter_mut().take(32).for_each(|v| *v = 0x11);
        vram.iter_mut().skip(32).take(32).for_each(|v| *v = 0x22);

        vram[0] = 0x00;
        vram[1] = 0x00;
        vram[2] = 0x01;
        vram[3] = 0x00;

        ppu.scanline = 0;
        ppu.step(1232, &vram);
    }

    #[test]
    fn test_mode0_transparency() {
        let mut ppu = PPU::new();
        ppu.dispcnt = 0x0100;
        ppu.bg_control[0] = BgControl {
            priority: 0,
            char_base: 0,
            mosaic: false,
            palette_256: false,
            screen_base: 8,
            wrap: false,
            screen_size: 0,
        };

        let mut vram = vec![0u8; 96 * 1024];
        ppu.palette_ram[2] = 0xFF;
        ppu.palette_ram[3] = 0x7F;

        vram[0] = 0x01;
        vram[1] = 0x10;

        let tilemap_offset = 16384;
        vram[tilemap_offset] = 0x00;
        vram[tilemap_offset + 1] = 0x00;

        ppu.scanline = 0;
        ppu.step(1232, &vram);

        assert_eq!(ppu.framebuffer[0], 0x7FFF);
        assert_eq!(ppu.framebuffer[1], 0x0000);
        assert_eq!(ppu.framebuffer[2], 0x0000);
        assert_eq!(ppu.framebuffer[3], 0x7FFF);
    }

    #[test]
    fn test_sprite_attribute_parsing() {
        let oam = vec![30, 0x00, 50, 0x40, 0x05, 0x20];

        let sprite = SpriteAttribute::from_oam_bytes(&oam);

        assert_eq!(sprite.y, 30);
        assert_eq!(sprite.x, 50);
        assert_eq!(sprite.tile_index, 5);
        assert_eq!(sprite.priority, 0);
        assert_eq!(sprite.palette_bank, 2);
        assert_eq!(sprite.get_size(), (16, 16));
        assert!(sprite.is_visible());
    }

    #[test]
    fn test_sprite_sizes() {
        let test_cases = [
            (0, 0, (8, 8)),
            (0, 1, (16, 16)),
            (0, 2, (32, 32)),
            (0, 3, (64, 64)),
            (1, 0, (16, 8)),
            (1, 1, (32, 8)),
            (1, 2, (32, 16)),
            (1, 3, (64, 32)),
            (2, 0, (8, 16)),
            (2, 1, (8, 32)),
            (2, 2, (16, 32)),
            (2, 3, (32, 64)),
        ];

        for (shape, size, expected) in test_cases {
            let sprite = SpriteAttribute {
                shape,
                size,
                ..SpriteAttribute::default()
            };
            assert_eq!(sprite.get_size(), expected);
        }
    }

    #[test]
    fn test_oam_read_write() {
        let mut ppu = PPU::new();

        ppu.write_oam_halfword(0, 0x0050);
        ppu.write_oam_halfword(2, 0x0064);
        ppu.write_oam_halfword(4, 0x000A);

        let sprite = ppu.read_sprite(0);
        assert_eq!(sprite.y, 80);
        assert_eq!(sprite.x, 100);
        assert_eq!(sprite.tile_index, 10);
    }

    #[test]
    fn test_sprite_rendering_simple() {
        let mut ppu = PPU::new();
        ppu.dispcnt = 0x1000;

        ppu.palette_ram[ppu_impl::OBJ_PALETTE_OFFSET + 2] = 0x00;
        ppu.palette_ram[ppu_impl::OBJ_PALETTE_OFFSET + 3] = 0x7C;

        ppu.write_oam_halfword(0, 0x0005);
        ppu.write_oam_halfword(2, 0x000A);
        ppu.write_oam_halfword(4, 0x0000);

        let mut vram = vec![0u8; 96 * 1024];
        let tile_offset = ppu_impl::OBJ_TILE_BASE;
        vram.iter_mut()
            .skip(tile_offset)
            .take(32)
            .for_each(|v| *v = 0x11);

        ppu.scanline = 5;
        ppu.step(1232, &vram);

        assert_eq!(
            ppu.framebuffer[5 * SCREEN_WIDTH + 10],
            0x7C00,
            "Sprite pixel should be blue"
        );
        assert_eq!(
            ppu.framebuffer[5 * SCREEN_WIDTH + 17],
            0x7C00,
            "Sprite extends to X=17"
        );
    }

    #[test]
    fn test_sprite_transparency() {
        let mut ppu = PPU::new();
        ppu.dispcnt = 0x1000;

        ppu.palette_ram[ppu_impl::OBJ_PALETTE_OFFSET + 2] = 0xFF;
        ppu.palette_ram[ppu_impl::OBJ_PALETTE_OFFSET + 3] = 0x7F;

        ppu.write_oam_halfword(0, 0x0000);
        ppu.write_oam_halfword(2, 0x0000);
        ppu.write_oam_halfword(4, 0x0000);

        let mut vram = vec![0u8; 96 * 1024];
        let tile_offset = ppu_impl::OBJ_TILE_BASE;
        vram[tile_offset] = 0x01;
        vram[tile_offset + 1] = 0x10;

        ppu.scanline = 0;
        ppu.step(1232, &vram);

        assert_eq!(ppu.framebuffer[0], 0x7FFF);
        assert_eq!(ppu.framebuffer[1], 0x0000);
        assert_eq!(ppu.framebuffer[3], 0x7FFF);
    }
}
