/// Display modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Mode0 = 0, // Tiled mode (4 backgrounds)
    Mode1 = 1, // Tiled mode (2 backgrounds + 1 affine)
    Mode2 = 2, // Tiled mode (2 affine backgrounds)
    Mode3 = 3, // Bitmap 240x160, 16-bit color
    Mode4 = 4, // Bitmap 240x160, 8-bit paletted
    Mode5 = 5, // Bitmap 160x128, 16-bit color
}

/// Background Control Register
#[derive(Debug, Clone, Copy, Default)]
pub struct BgControl {
    pub priority: u8,      // Bits 0-1
    pub char_base: u8,     // Bits 2-3 (character base block * 16KB)
    pub mosaic: bool,      // Bit 6
    pub palette_256: bool, // Bit 7 (false = 16x16, true = 256x1)
    pub screen_base: u8,   // Bits 8-12 (screen base block * 2KB)
    pub wrap: bool,        // Bit 13 (affine wrap)
    pub screen_size: u8,   // Bits 14-15
}

impl BgControl {
    pub fn from_u16(value: u16) -> Self {
        Self {
            priority: (value & 0x3) as u8,
            char_base: ((value >> 2) & 0x3) as u8,
            mosaic: (value & (1 << 6)) != 0,
            palette_256: (value & (1 << 7)) != 0,
            screen_base: ((value >> 8) & 0x1F) as u8,
            wrap: (value & (1 << 13)) != 0,
            screen_size: ((value >> 14) & 0x3) as u8,
        }
    }

    pub fn to_u16(&self) -> u16 {
        (self.priority as u16)
            | ((self.char_base as u16) << 2)
            | ((self.mosaic as u16) << 6)
            | ((self.palette_256 as u16) << 7)
            | ((self.screen_base as u16) << 8)
            | ((self.wrap as u16) << 13)
            | ((self.screen_size as u16) << 14)
    }

    /// Get screen size in tiles (width, height)
    pub fn get_screen_size(&self) -> (usize, usize) {
        match self.screen_size {
            0 => (32, 32), // 256x256 px
            1 => (64, 32), // 512x256 px
            2 => (32, 64), // 256x512 px
            3 => (64, 64), // 512x512 px
            _ => (32, 32),
        }
    }
}
