/// PPU Mode 5 - Bitmap RGB (160x128, 16-bit direct color)
///
/// Mode 5 uses 16-bit RGB555 direct color on a smaller screen.
/// Resolution: 160x128 pixels
/// Two frame buffers for page flipping (double buffering).
/// Frame 0: 0x06000000-0x06005000 (40960 bytes = 160*128*2)
/// Frame 1: 0x0600A000-0x0600F000 (40960 bytes)
use super::constants::*;

/// Mode 5 screen dimensions
pub const MODE5_WIDTH: usize = 160;
pub const MODE5_HEIGHT: usize = 128;

/// Render Mode 5 scanline - 16-bit RGB bitmap
pub fn render_mode5_scanline(
    framebuffer: &mut [u16],
    vram: &[u8],
    scanline: usize,
    frame_select: bool,
) {
    // Only render if within Mode 5 bounds
    if scanline >= MODE5_HEIGHT {
        // Fill rest of screen with black
        let line_offset = scanline * SCREEN_WIDTH;
        for x in 0..SCREEN_WIDTH {
            framebuffer[line_offset + x] = 0;
        }
        return;
    }

    // Page flip: frame 0 or frame 1
    let frame_offset = if frame_select { 0xA000 } else { 0x0000 };

    let line_offset = scanline * SCREEN_WIDTH;

    // Center 160x128 image on 240x160 screen
    let x_offset = (SCREEN_WIDTH - MODE5_WIDTH) / 2; // 40 pixels border left/right

    // Black borders on left
    for x in 0..x_offset {
        framebuffer[line_offset + x] = 0;
    }

    // Render Mode 5 pixels (160 wide)
    for x in 0..MODE5_WIDTH {
        let vram_addr = frame_offset + (scanline * MODE5_WIDTH + x) * 2;

        // Read 16-bit RGB555 color directly from VRAM
        if vram_addr + 1 < vram.len() {
            let color_low = vram[vram_addr] as u16;
            let color_high = vram[vram_addr + 1] as u16;
            let rgb555 = color_low | (color_high << 8);
            framebuffer[line_offset + x_offset + x] = rgb555;
        } else {
            framebuffer[line_offset + x_offset + x] = 0;
        }
    }

    // Black borders on right
    for x in (x_offset + MODE5_WIDTH)..SCREEN_WIDTH {
        framebuffer[line_offset + x] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode5_basic_render() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000];

        // Red pixel at (0,0) - offset by 40 pixels due to centering
        vram[0] = 0x1F; // Red low byte
        vram[1] = 0x00; // Red high byte

        // Green pixel at (1,0)
        vram[2] = 0xE0; // Green low byte
        vram[3] = 0x03; // Green high byte

        render_mode5_scanline(&mut framebuffer, &vram, 0, false);

        let x_offset = (SCREEN_WIDTH - MODE5_WIDTH) / 2;
        assert_eq!(framebuffer[x_offset], 0x001F); // Red
        assert_eq!(framebuffer[x_offset + 1], 0x03E0); // Green
        assert_eq!(framebuffer[0], 0); // Left border black
        assert_eq!(framebuffer[SCREEN_WIDTH - 1], 0); // Right border black
    }

    #[test]
    fn test_mode5_page_flip() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000];

        // Frame 0: blue at (0,0)
        vram[0] = 0x00;
        vram[1] = 0x7C; // 0x7C00 = blue

        // Frame 1: white at (0,0) - offset 0xA000
        vram[0xA000] = 0xFF;
        vram[0xA001] = 0x7F; // 0x7FFF = white

        let x_offset = (SCREEN_WIDTH - MODE5_WIDTH) / 2;

        // Render frame 0
        render_mode5_scanline(&mut framebuffer, &vram, 0, false);
        assert_eq!(framebuffer[x_offset], 0x7C00); // Blue

        // Clear and render frame 1
        framebuffer.fill(0);
        render_mode5_scanline(&mut framebuffer, &vram, 0, true);
        assert_eq!(framebuffer[x_offset], 0x7FFF); // White
    }

    #[test]
    fn test_mode5_dimensions() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000];

        // Fill entire Mode 5 screen with white
        for y in 0..MODE5_HEIGHT {
            for x in 0..MODE5_WIDTH {
                let addr = (y * MODE5_WIDTH + x) * 2;
                vram[addr] = 0xFF;
                vram[addr + 1] = 0x7F; // White
            }
        }

        // Render first scanline
        render_mode5_scanline(&mut framebuffer, &vram, 0, false);

        let x_offset = (SCREEN_WIDTH - MODE5_WIDTH) / 2;

        // Check borders are black
        assert_eq!(framebuffer[0], 0);
        assert_eq!(framebuffer[x_offset - 1], 0);
        assert_eq!(framebuffer[x_offset + MODE5_WIDTH], 0);
        assert_eq!(framebuffer[SCREEN_WIDTH - 1], 0);

        // Check content is white
        for x in 0..MODE5_WIDTH {
            assert_eq!(framebuffer[x_offset + x], 0x7FFF);
        }
    }

    #[test]
    fn test_mode5_out_of_bounds() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let vram = vec![0u8; 0x18000];

        // Render scanline beyond Mode 5 height (128)
        render_mode5_scanline(&mut framebuffer, &vram, 150, false);

        // Entire line should be black
        for x in 0..SCREEN_WIDTH {
            assert_eq!(framebuffer[150 * SCREEN_WIDTH + x], 0);
        }
    }

    #[test]
    fn test_mode5_centering() {
        // Verify centering calculation
        let x_offset = (SCREEN_WIDTH - MODE5_WIDTH) / 2;
        assert_eq!(x_offset, 40); // (240 - 160) / 2 = 40

        // 40 pixels black border on left
        // 160 pixels content
        // 40 pixels black border on right
        // Total: 240 pixels
        assert_eq!(x_offset + MODE5_WIDTH + x_offset, SCREEN_WIDTH);
    }

    #[test]
    fn test_mode5_gradient() {
        let mut framebuffer = vec![0u16; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut vram = vec![0u8; 0x18000];

        // Create horizontal gradient on scanline 0
        for x in 0..MODE5_WIDTH {
            let intensity = (x * 31 / MODE5_WIDTH) as u16;
            let color = intensity | (intensity << 5) | (intensity << 10);
            let addr = x * 2;
            vram[addr] = (color & 0xFF) as u8;
            vram[addr + 1] = ((color >> 8) & 0xFF) as u8;
        }

        render_mode5_scanline(&mut framebuffer, &vram, 0, false);

        let x_offset = (SCREEN_WIDTH - MODE5_WIDTH) / 2;

        // Verify gradient (first pixel dark, last pixel bright)
        assert!(framebuffer[x_offset] < framebuffer[x_offset + MODE5_WIDTH - 1]);
    }
}
