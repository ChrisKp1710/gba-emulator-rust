use super::constants::SCREEN_WIDTH;

/// Render scanline in Mode 3 (16-bit bitmap)
pub fn render_mode3_scanline(scanline: u16, vram: &[u8], framebuffer: &mut [u16]) {
    // Mode 3: VRAM is array of u16 (RGB555)
    // Offset = scanline * width * 2 bytes
    let line = scanline as usize;
    let offset = line * SCREEN_WIDTH * 2;

    // Copy scanline from VRAM to framebuffer
    for x in 0..SCREEN_WIDTH {
        let vram_idx = offset + x * 2;

        // Read RGB555 pixel (little endian)
        if vram_idx + 1 < vram.len() {
            let pixel = (vram[vram_idx] as u16) | ((vram[vram_idx + 1] as u16) << 8);
            framebuffer[line * SCREEN_WIDTH + x] = pixel;
        } else {
            // Out of bounds, black pixel
            framebuffer[line * SCREEN_WIDTH + x] = 0;
        }
    }
}
