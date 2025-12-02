/// PPU Blending - Alpha blending and brightness effects
///
/// GBA supports 3 blending modes:
/// - Mode 0: None (disabled)
/// - Mode 1: Alpha blending (blend two layers)
/// - Mode 2: Brightness increase (fade to white)
/// - Mode 3: Brightness decrease (fade to black)
///
/// Registers:
/// - BLDCNT: Blend control (which layers to blend)
/// - BLDALPHA: Alpha coefficients (EVA, EVB)
/// - BLDY: Brightness coefficient (EVY)

/// Blend mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendMode {
    None = 0,
    AlphaBlend = 1,
    BrightnessIncrease = 2,
    BrightnessDecrease = 3,
}

impl BlendMode {
    pub fn from_u16(value: u16) -> Self {
        match (value >> 6) & 0x3 {
            1 => BlendMode::AlphaBlend,
            2 => BlendMode::BrightnessIncrease,
            3 => BlendMode::BrightnessDecrease,
            _ => BlendMode::None,
        }
    }
}

/// Blend control register (BLDCNT)
#[derive(Debug, Clone, Copy)]
pub struct BlendControl {
    pub mode: BlendMode,
    // Target 1 (top layer)
    pub bg0_target1: bool,
    pub bg1_target1: bool,
    pub bg2_target1: bool,
    pub bg3_target1: bool,
    pub obj_target1: bool,
    pub backdrop_target1: bool,
    // Target 2 (bottom layer, for alpha blend)
    pub bg0_target2: bool,
    pub bg1_target2: bool,
    pub bg2_target2: bool,
    pub bg3_target2: bool,
    pub obj_target2: bool,
    pub backdrop_target2: bool,
}

impl BlendControl {
    pub fn new() -> Self {
        Self {
            mode: BlendMode::None,
            bg0_target1: false,
            bg1_target1: false,
            bg2_target1: false,
            bg3_target1: false,
            obj_target1: false,
            backdrop_target1: false,
            bg0_target2: false,
            bg1_target2: false,
            bg2_target2: false,
            bg3_target2: false,
            obj_target2: false,
            backdrop_target2: false,
        }
    }

    pub fn from_u16(value: u16) -> Self {
        Self {
            mode: BlendMode::from_u16(value),
            bg0_target1: (value & 0x01) != 0,
            bg1_target1: (value & 0x02) != 0,
            bg2_target1: (value & 0x04) != 0,
            bg3_target1: (value & 0x08) != 0,
            obj_target1: (value & 0x10) != 0,
            backdrop_target1: (value & 0x20) != 0,
            bg0_target2: (value & 0x0100) != 0,
            bg1_target2: (value & 0x0200) != 0,
            bg2_target2: (value & 0x0400) != 0,
            bg3_target2: (value & 0x0800) != 0,
            obj_target2: (value & 0x1000) != 0,
            backdrop_target2: (value & 0x2000) != 0,
        }
    }

    pub fn to_u16(&self) -> u16 {
        ((self.mode as u16) << 6)
            | ((self.bg0_target1 as u16) << 0)
            | ((self.bg1_target1 as u16) << 1)
            | ((self.bg2_target1 as u16) << 2)
            | ((self.bg3_target1 as u16) << 3)
            | ((self.obj_target1 as u16) << 4)
            | ((self.backdrop_target1 as u16) << 5)
            | ((self.bg0_target2 as u16) << 8)
            | ((self.bg1_target2 as u16) << 9)
            | ((self.bg2_target2 as u16) << 10)
            | ((self.bg3_target2 as u16) << 11)
            | ((self.obj_target2 as u16) << 12)
            | ((self.backdrop_target2 as u16) << 13)
    }
}

/// Alpha blending coefficients
#[derive(Debug, Clone, Copy)]
pub struct AlphaCoefficients {
    pub eva: u8, // Target 1 coefficient (0-16)
    pub evb: u8, // Target 2 coefficient (0-16)
}

impl AlphaCoefficients {
    pub fn from_u16(value: u16) -> Self {
        let eva = (value & 0x1F) as u8;
        let evb = ((value >> 8) & 0x1F) as u8;
        Self {
            eva: eva.min(16),
            evb: evb.min(16),
        }
    }

    pub fn to_u16(&self) -> u16 {
        (self.eva as u16) | ((self.evb as u16) << 8)
    }
}

/// Blend two RGB555 colors using alpha coefficients
#[allow(dead_code)]
pub fn alpha_blend(color1: u16, color2: u16, eva: u8, evb: u8) -> u16 {
    let r1 = (color1 & 0x1F) as u32;
    let g1 = ((color1 >> 5) & 0x1F) as u32;
    let b1 = ((color1 >> 10) & 0x1F) as u32;

    let r2 = (color2 & 0x1F) as u32;
    let g2 = ((color2 >> 5) & 0x1F) as u32;
    let b2 = ((color2 >> 10) & 0x1F) as u32;

    // Formula: (color1 * EVA + color2 * EVB) / 16
    let r = ((r1 * eva as u32 + r2 * evb as u32) / 16).min(31);
    let g = ((g1 * eva as u32 + g2 * evb as u32) / 16).min(31);
    let b = ((b1 * eva as u32 + b2 * evb as u32) / 16).min(31);

    (r as u16) | ((g as u16) << 5) | ((b as u16) << 10)
}

/// Increase brightness (fade to white)
#[allow(dead_code)]
pub fn brightness_increase(color: u16, evy: u8) -> u16 {
    let r = (color & 0x1F) as u32;
    let g = ((color >> 5) & 0x1F) as u32;
    let b = ((color >> 10) & 0x1F) as u32;

    // Formula: color + (31 - color) * EVY / 16
    let r = (r + ((31 - r) * evy as u32) / 16).min(31);
    let g = (g + ((31 - g) * evy as u32) / 16).min(31);
    let b = (b + ((31 - b) * evy as u32) / 16).min(31);

    ((b << 10) | (g << 5) | r) as u16
}

/// Decrease brightness (fade to black)
#[allow(dead_code)]
pub fn brightness_decrease(color: u16, evy: u8) -> u16 {
    let r = (color & 0x1F) as u32;
    let g = ((color >> 5) & 0x1F) as u32;
    let b = ((color >> 10) & 0x1F) as u32;

    // Formula: color - color * EVY / 16
    let r = r.saturating_sub((r * evy as u32) / 16);
    let g = g.saturating_sub((g * evy as u32) / 16);
    let b = b.saturating_sub((b * evy as u32) / 16);

    (r as u16) | ((g as u16) << 5) | ((b as u16) << 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_mode_parsing() {
        assert_eq!(BlendMode::from_u16(0b00000000), BlendMode::None);
        assert_eq!(BlendMode::from_u16(0b01000000), BlendMode::AlphaBlend);
        assert_eq!(
            BlendMode::from_u16(0b10000000),
            BlendMode::BrightnessIncrease
        );
        assert_eq!(
            BlendMode::from_u16(0b11000000),
            BlendMode::BrightnessDecrease
        );
    }

    #[test]
    fn test_blend_control_parsing() {
        // Binary: 0b0010001101000011
        // Mode (bits 6-7): 01 = Alpha blend
        // Target1 (bits 0-5): 0b000011 = BG0, BG1
        // Target2 (bits 8-13): 0b001001 = BG0, BG3
        let ctrl = BlendControl::from_u16(0b0010010101000011);
        assert_eq!(ctrl.mode, BlendMode::AlphaBlend);
        // Target 1: BG0, BG1
        assert!(ctrl.bg0_target1);
        assert!(ctrl.bg1_target1);
        assert!(!ctrl.bg2_target1);
        // Target 2: BG0, BG2
        assert!(ctrl.bg0_target2);
        assert!(!ctrl.bg1_target2);
        assert!(ctrl.bg2_target2);

        assert_eq!(ctrl.to_u16(), 0b0010010101000011);
    }

    #[test]
    fn test_alpha_coefficients() {
        let coeff = AlphaCoefficients::from_u16(0x0A08);
        assert_eq!(coeff.eva, 8);
        assert_eq!(coeff.evb, 10);
        assert_eq!(coeff.to_u16(), 0x0A08);

        // Clamping to 16
        let coeff = AlphaCoefficients::from_u16(0x1F1F);
        assert_eq!(coeff.eva, 16);
        assert_eq!(coeff.evb, 16);
    }

    #[test]
    fn test_alpha_blend_50_50() {
        // Red (0x001F) + Blue (0x7C00) with EVA=8, EVB=8 (50%/50%)
        let red = 0x001F;
        let blue = 0x7C00;
        let result = alpha_blend(red, blue, 8, 8);

        // Expected: (31*8 + 0*8)/16 = 15 for red
        //           (0*8 + 31*8)/16 = 15 for blue
        let expected_r = 15;
        let expected_b = 15;
        assert_eq!(result & 0x1F, expected_r);
        assert_eq!((result >> 10) & 0x1F, expected_b);
    }

    #[test]
    fn test_alpha_blend_75_25() {
        // Green (0x03E0) + Red (0x001F) with EVA=12, EVB=4 (75%/25%)
        let green = 0x03E0;
        let red = 0x001F;
        let result = alpha_blend(green, red, 12, 4);

        // Red: (0*12 + 31*4)/16 = 7
        // Green: (31*12 + 0*4)/16 = 23
        assert_eq!(result & 0x1F, 7);
        assert_eq!((result >> 5) & 0x1F, 23);
    }

    #[test]
    fn test_brightness_increase() {
        // Black to white
        let black = 0x0000;
        let result = brightness_increase(black, 16); // Max brightness
        assert_eq!(result, 0x7FFF); // White

        // Half brightness
        let result = brightness_increase(black, 8);
        // (0 + (31 - 0) * 8 / 16) = 15
        assert_eq!(result & 0x1F, 15);
        assert_eq!((result >> 5) & 0x1F, 15);
        assert_eq!((result >> 10) & 0x1F, 15);

        // Already white - no change
        let white = 0x7FFF;
        let result = brightness_increase(white, 16);
        assert_eq!(result, white);
    }

    #[test]
    fn test_brightness_decrease() {
        // White to black
        let white = 0x7FFF;
        let result = brightness_decrease(white, 16); // Max darkness
        assert_eq!(result, 0x0000); // Black

        // Half darkness
        let result = brightness_decrease(white, 8);
        // (31 - 31 * 8 / 16) = 16 (integer rounding)
        assert_eq!(result & 0x1F, 16);
        assert_eq!((result >> 5) & 0x1F, 16);
        assert_eq!((result >> 10) & 0x1F, 16);

        // Already black - no change
        let black = 0x0000;
        let result = brightness_decrease(black, 16);
        assert_eq!(result, black);
    }

    #[test]
    fn test_brightness_gradient() {
        let color = 0x03E0; // Green

        // EVY=0 should not change color
        assert_eq!(brightness_increase(color, 0), color);
        assert_eq!(brightness_decrease(color, 0), color);

        // EVY=16 should go to white/black
        assert_eq!(brightness_increase(color, 16), 0x7FFF);
        assert_eq!(brightness_decrease(color, 16), 0x0000);
    }

    #[test]
    fn test_alpha_blend_no_overflow() {
        // Max values should not overflow
        let white = 0x7FFF;
        let result = alpha_blend(white, white, 16, 16);
        assert_eq!(result, white); // Should clamp to white

        // Test individual channels don't exceed 31
        let result = alpha_blend(0x7FFF, 0x7FFF, 16, 16);
        assert!((result & 0x1F) <= 31);
        assert!(((result >> 5) & 0x1F) <= 31);
        assert!(((result >> 10) & 0x1F) <= 31);
    }
}
