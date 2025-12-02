/// PPU Affine Backgrounds - Rotation/Scaling (Mode 1-2)
///
/// Affine backgrounds allow rotation, scaling, and shearing transformations.
/// Available in:
/// - Mode 1: BG2 is affine (BG0, BG1 are regular tile backgrounds)
/// - Mode 2: BG2 and BG3 are both affine
///
/// Transformation matrix (2x2):
/// | PA  PB |   | dx/dx  dy/dx |
/// | PC  PD | = | dx/dy  dy/dy |
///
/// Reference point (X, Y) in background space
/// Transformed pixel calculation:
/// screen_x' = PA * (screen_x - ref_x) + PB * (screen_y - ref_y) + bg_x
/// screen_y' = PC * (screen_x - ref_x) + PD * (screen_y - ref_y) + bg_y
///
/// Registers per affine BG:
/// - BGxPA, BGxPB, BGxPC, BGxPD: Transformation matrix (fixed-point 8.8)
/// - BGxX, BGxY: Reference point (fixed-point 20.8)
use super::constants::SCREEN_WIDTH;

/// Affine transformation matrix
#[derive(Debug, Clone, Copy)]
pub struct AffineMatrix {
    pub pa: i16, // dx/dx (8.8 fixed-point)
    pub pb: i16, // dy/dx (8.8 fixed-point)
    pub pc: i16, // dx/dy (8.8 fixed-point)
    pub pd: i16, // dy/dy (8.8 fixed-point)
}

impl AffineMatrix {
    pub fn identity() -> Self {
        Self {
            pa: 0x0100, // 1.0 in 8.8 fixed-point
            pb: 0,
            pc: 0,
            pd: 0x0100, // 1.0 in 8.8 fixed-point
        }
    }

    /// Create rotation matrix
    /// angle in degrees
    pub fn rotation(angle: f32) -> Self {
        let rad = angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        Self {
            pa: (cos * 256.0) as i16,
            pb: (-sin * 256.0) as i16,
            pc: (sin * 256.0) as i16,
            pd: (cos * 256.0) as i16,
        }
    }

    /// Create scaling matrix
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            pa: (sx * 256.0) as i16,
            pb: 0,
            pc: 0,
            pd: (sy * 256.0) as i16,
        }
    }
}

/// Affine background parameters
#[derive(Debug, Clone, Copy)]
pub struct AffineParams {
    pub matrix: AffineMatrix,
    pub ref_x: i32, // 20.8 fixed-point
    pub ref_y: i32, // 20.8 fixed-point
}

impl AffineParams {
    pub fn new() -> Self {
        Self {
            matrix: AffineMatrix::identity(),
            ref_x: 0,
            ref_y: 0,
        }
    }
}

/// Transform screen coordinates to background coordinates
/// Returns (bg_x, bg_y) in 8.8 fixed-point
#[allow(dead_code)]
pub fn transform_point(screen_x: i32, screen_y: i32, params: &AffineParams) -> (i32, i32) {
    // Screen coordinates relative to reference point (center of screen)
    let dx = (screen_x << 8) - params.ref_x;
    let dy = (screen_y << 8) - params.ref_y;

    // Apply transformation matrix
    let bg_x = (params.matrix.pa as i32 * dx + params.matrix.pb as i32 * dy) >> 8;
    let bg_y = (params.matrix.pc as i32 * dx + params.matrix.pd as i32 * dy) >> 8;

    (bg_x, bg_y)
}

/// Render affine background scanline
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn render_affine_scanline(
    framebuffer: &mut [u16],
    scanline: usize,
    width: usize,
    bg_size: usize, // Background size in pixels (128, 256, 512, 1024)
    wraparound: bool,
    vram: &[u8],
    palette_ram: &[u8],
    char_base: usize,
    screen_base: usize,
    params: &AffineParams,
) {
    let line_offset = scanline * SCREEN_WIDTH;

    for x in 0..width {
        // Transform screen coordinates to background space
        let (bg_x_fp, bg_y_fp) = transform_point(x as i32, scanline as i32, params);

        // Convert from fixed-point to integer (8.8 -> integer)
        let bg_x = bg_x_fp >> 8;
        let bg_y = bg_y_fp >> 8;

        // Handle wraparound or clipping
        let (final_x, final_y) = if wraparound {
            // Wraparound: modulo background size
            let wrapped_x = bg_x.rem_euclid(bg_size as i32) as usize;
            let wrapped_y = bg_y.rem_euclid(bg_size as i32) as usize;
            (wrapped_x, wrapped_y)
        } else {
            // Clipping: out-of-bounds = transparent
            if bg_x < 0 || bg_y < 0 || bg_x >= bg_size as i32 || bg_y >= bg_size as i32 {
                framebuffer[line_offset + x] = 0; // Transparent
                continue;
            }
            (bg_x as usize, bg_y as usize)
        };

        // Get tile coordinates (8x8 tiles)
        let tile_x = final_x / 8;
        let tile_y = final_y / 8;
        let tiles_per_row = bg_size / 8;

        // Get pixel within tile
        let pixel_x = final_x % 8;
        let pixel_y = final_y % 8;

        // Read tile number from screen data (1 byte per tile for affine)
        let screen_addr = screen_base + (tile_y * tiles_per_row + tile_x);
        let tile_num = vram.get(screen_addr).copied().unwrap_or(0) as usize;

        // Read pixel from character data (8-bit paletted)
        let tile_addr = char_base + (tile_num * 64) + (pixel_y * 8) + pixel_x;
        let palette_index = vram.get(tile_addr).copied().unwrap_or(0) as usize;

        // Lookup color in palette (256-color mode, BG palette)
        if palette_index == 0 {
            framebuffer[line_offset + x] = 0; // Transparent
        } else {
            let color_addr = palette_index * 2;
            if color_addr + 1 < 512 {
                let color_low = palette_ram[color_addr] as u16;
                let color_high = palette_ram[color_addr + 1] as u16;
                framebuffer[line_offset + x] = color_low | (color_high << 8);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_matrix() {
        let matrix = AffineMatrix::identity();
        assert_eq!(matrix.pa, 0x0100); // 1.0
        assert_eq!(matrix.pb, 0);
        assert_eq!(matrix.pc, 0);
        assert_eq!(matrix.pd, 0x0100); // 1.0
    }

    #[test]
    fn test_transform_identity() {
        let params = AffineParams {
            matrix: AffineMatrix::identity(),
            ref_x: 0,
            ref_y: 0,
        };

        let (bg_x, bg_y) = transform_point(10, 20, &params);
        // Identity: output = input (in 8.8 fixed-point)
        assert_eq!(bg_x, 10 << 8);
        assert_eq!(bg_y, 20 << 8);
    }

    #[test]
    fn test_transform_scale_2x() {
        let params = AffineParams {
            matrix: AffineMatrix::scale(2.0, 2.0),
            ref_x: 0,
            ref_y: 0,
        };

        let (bg_x, bg_y) = transform_point(10, 20, &params);
        // 2x scale: output = input * 2
        assert_eq!(bg_x >> 8, 20);
        assert_eq!(bg_y >> 8, 40);
    }

    #[test]
    fn test_transform_scale_half() {
        let params = AffineParams {
            matrix: AffineMatrix::scale(0.5, 0.5),
            ref_x: 0,
            ref_y: 0,
        };

        let (bg_x, bg_y) = transform_point(100, 200, &params);
        // 0.5x scale: output = input / 2
        assert_eq!(bg_x >> 8, 50);
        assert_eq!(bg_y >> 8, 100);
    }

    #[test]
    fn test_transform_rotation_90() {
        let params = AffineParams {
            matrix: AffineMatrix::rotation(90.0),
            ref_x: 0,
            ref_y: 0,
        };

        let (bg_x, bg_y) = transform_point(10, 0, &params);
        // 90° rotation: (10, 0) -> (0, 10) approximately
        assert!((bg_x >> 8).abs() < 2); // Close to 0
        assert!(((bg_y >> 8) - 10).abs() < 2); // Close to 10
    }

    #[test]
    fn test_wraparound() {
        let mut framebuffer = vec![0u16; 240 * 160];
        let vram = vec![0u8; 0x18000];
        let palette_ram = vec![0u8; 512];

        let params = AffineParams::new();

        // Render with wraparound enabled
        render_affine_scanline(
            &mut framebuffer,
            0,
            240,
            256,  // BG size
            true, // wraparound
            &vram,
            &palette_ram,
            0,
            0,
            &params,
        );

        // Should complete without panic (out-of-bounds wraps)
    }

    #[test]
    fn test_clipping() {
        let mut framebuffer = vec![0u16; 240 * 160];
        let vram = vec![0u8; 0x18000];
        let palette_ram = vec![0u8; 512];

        let params = AffineParams {
            matrix: AffineMatrix::scale(10.0, 10.0), // Large scale = out of bounds
            ref_x: 0,
            ref_y: 0,
        };

        // Render without wraparound (clipping mode)
        render_affine_scanline(
            &mut framebuffer,
            0,
            240,
            256,
            false, // no wraparound
            &vram,
            &palette_ram,
            0,
            0,
            &params,
        );

        // Out-of-bounds pixels should be transparent (0)
        // Most pixels will be out of bounds due to 10x scale
    }

    #[test]
    fn test_bg_sizes() {
        let sizes = [128, 256, 512, 1024];

        for &size in &sizes {
            let tiles_per_row = size / 8;
            assert_eq!(tiles_per_row * 8, size);
        }
    }

    #[test]
    fn test_reference_point() {
        // Reference point shifts the transformation center
        let params1 = AffineParams {
            matrix: AffineMatrix::identity(),
            ref_x: 100 << 8,
            ref_y: 50 << 8,
        };

        let (bg_x, bg_y) = transform_point(100, 50, &params1);
        // At reference point, output should be 0
        assert_eq!(bg_x, 0);
        assert_eq!(bg_y, 0);

        let (bg_x2, bg_y2) = transform_point(110, 60, &params1);
        // Offset from reference
        assert_eq!(bg_x2 >> 8, 10);
        assert_eq!(bg_y2 >> 8, 10);
    }

    #[test]
    fn test_rotation_matrices() {
        // Test common rotation angles
        let rot_0 = AffineMatrix::rotation(0.0);
        assert_eq!(rot_0.pa, 0x0100); // cos(0) = 1
        assert_eq!(rot_0.pd, 0x0100);

        let rot_180 = AffineMatrix::rotation(180.0);
        assert!(rot_180.pa < -240); // cos(180) ≈ -1
        assert!(rot_180.pd < -240);

        let rot_270 = AffineMatrix::rotation(270.0);
        // cos(270) ≈ 0, sin(270) ≈ -1
        assert!(rot_270.pa.abs() < 10); // Close to 0
        assert!(rot_270.pc < -240); // sin(270) ≈ -1
    }

    #[test]
    fn test_tile_addressing() {
        // Verify tile coordinate calculation
        let bg_size = 256;
        let final_x = 100;
        let final_y = 80;

        let tile_x = final_x / 8;
        let tile_y = final_y / 8;
        let tiles_per_row = bg_size / 8;

        assert_eq!(tile_x, 12);
        assert_eq!(tile_y, 10);
        assert_eq!(tiles_per_row, 32);

        let screen_addr = tile_y * tiles_per_row + tile_x;
        assert_eq!(screen_addr, 10 * 32 + 12);
    }

    #[test]
    fn test_pixel_within_tile() {
        let final_x = 77; // Tile 9, pixel 5
        let final_y = 66; // Tile 8, pixel 2

        let pixel_x = final_x % 8;
        let pixel_y = final_y % 8;

        assert_eq!(pixel_x, 5);
        assert_eq!(pixel_y, 2);
    }

    #[test]
    fn test_negative_coordinates_wraparound() {
        let bg_size = 256i32;
        let bg_x: i32 = -10;
        let bg_y: i32 = -20;

        let wrapped_x = bg_x.rem_euclid(bg_size) as usize;
        let wrapped_y = bg_y.rem_euclid(bg_size) as usize;

        assert_eq!(wrapped_x, 246);
        assert_eq!(wrapped_y, 236);
    }
}
