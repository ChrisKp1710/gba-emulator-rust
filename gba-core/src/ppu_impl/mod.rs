/// PPU - Picture Processing Unit
/// Modular implementation
mod blending;
mod constants;
mod mode0;
mod mode3;
mod mode4;
mod mode5;
mod sprites;
mod types;
mod windows;

pub use constants::*;
pub use sprites::SpriteAttribute;
pub use types::{BgControl, DisplayMode};

pub struct PPU {
    /// Frame buffer (RGB555 format: xBBBBBGGGGGRRRRR)
    pub framebuffer: Vec<u16>,

    /// Display Control Register (DISPCNT)
    pub dispcnt: u16,

    /// Display Status Register (DISPSTAT)
    pub dispstat: u16,

    /// Current scanline (VCOUNT)
    pub scanline: u16,

    /// Accumulated PPU cycles
    pub cycles: u32,

    /// Background Control Registers (BG0-BG3)
    pub bg_control: [BgControl; 4],

    /// Background Scroll X (BG0-BG3)
    pub bg_hofs: [u16; 4],

    /// Background Scroll Y (BG0-BG3)
    pub bg_vofs: [u16; 4],

    /// Palette RAM (1KB: 512 bytes BG + 512 bytes OBJ)
    pub palette_ram: Vec<u8>,

    /// OAM (Object Attribute Memory - 1KB, 128 sprites)
    pub oam: Vec<u8>,

    /// Window system
    pub windows: windows::Windows,

    /// Blend control
    pub blend_control: blending::BlendControl,

    /// Alpha coefficients (BLDALPHA)
    pub alpha_coefficients: blending::AlphaCoefficients,

    /// Brightness coefficient (BLDY)
    pub brightness_coeff: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            dispcnt: 0,
            dispstat: 0,
            scanline: 0,
            cycles: 0,
            bg_control: [BgControl::default(); 4],
            bg_hofs: [0; 4],
            bg_vofs: [0; 4],
            palette_ram: vec![0; PALETTE_RAM_SIZE],
            oam: vec![0; OAM_SIZE],
            windows: windows::Windows::new(),
            blend_control: blending::BlendControl::new(),
            alpha_coefficients: blending::AlphaCoefficients { eva: 0, evb: 0 },
            brightness_coeff: 0,
        }
    }

    /// Read I/O register
    pub fn read_register(&self, addr: u32) -> u16 {
        match addr {
            DISPCNT => self.dispcnt,
            DISPSTAT => self.dispstat | (self.in_vblank() as u16),
            VCOUNT => self.scanline,
            BG0CNT => self.bg_control[0].to_u16(),
            BG1CNT => self.bg_control[1].to_u16(),
            BG2CNT => self.bg_control[2].to_u16(),
            BG3CNT => self.bg_control[3].to_u16(),
            BG0HOFS => self.bg_hofs[0],
            BG0VOFS => self.bg_vofs[0],
            BG1HOFS => self.bg_hofs[1],
            BG1VOFS => self.bg_vofs[1],
            BG2HOFS => self.bg_hofs[2],
            BG2VOFS => self.bg_vofs[2],
            BG3HOFS => self.bg_hofs[3],
            BG3VOFS => self.bg_vofs[3],
            BLDCNT => self.blend_control.to_u16(),
            BLDALPHA => self.alpha_coefficients.to_u16(),
            _ => 0,
        }
    }

    /// Write I/O register
    pub fn write_register(&mut self, addr: u32, value: u16) {
        match addr {
            DISPCNT => {
                self.dispcnt = value;
            }
            DISPSTAT => {
                self.dispstat = (self.dispstat & 0x0007) | (value & 0xFFF8);
            }
            BG0CNT => self.bg_control[0] = BgControl::from_u16(value),
            BG1CNT => self.bg_control[1] = BgControl::from_u16(value),
            BG2CNT => self.bg_control[2] = BgControl::from_u16(value),
            BG3CNT => self.bg_control[3] = BgControl::from_u16(value),
            BG0HOFS => self.bg_hofs[0] = value & 0x1FF,
            BG0VOFS => self.bg_vofs[0] = value & 0x1FF,
            BG1HOFS => self.bg_hofs[1] = value & 0x1FF,
            BG1VOFS => self.bg_vofs[1] = value & 0x1FF,
            BG2HOFS => self.bg_hofs[2] = value & 0x1FF,
            BG2VOFS => self.bg_vofs[2] = value & 0x1FF,
            BG3HOFS => self.bg_hofs[3] = value & 0x1FF,
            BG3VOFS => self.bg_vofs[3] = value & 0x1FF,
            WIN0H => {
                let (left, right) = windows::WindowBounds::from_horizontal(value);
                self.windows.win0.left = left;
                self.windows.win0.right = right;
            }
            WIN1H => {
                let (left, right) = windows::WindowBounds::from_horizontal(value);
                self.windows.win1.left = left;
                self.windows.win1.right = right;
            }
            WIN0V => {
                let (top, bottom) = windows::WindowBounds::from_vertical(value);
                self.windows.win0.top = top;
                self.windows.win0.bottom = bottom;
            }
            WIN1V => {
                let (top, bottom) = windows::WindowBounds::from_vertical(value);
                self.windows.win1.top = top;
                self.windows.win1.bottom = bottom;
            }
            WININ => {
                self.windows.win0_control = windows::WindowControl::from_u8((value & 0xFF) as u8);
                self.windows.win1_control =
                    windows::WindowControl::from_u8(((value >> 8) & 0xFF) as u8);
            }
            WINOUT => {
                self.windows.winout_control = windows::WindowControl::from_u8((value & 0xFF) as u8);
                self.windows.winobj_control =
                    windows::WindowControl::from_u8(((value >> 8) & 0xFF) as u8);
            }
            BLDCNT => self.blend_control = blending::BlendControl::from_u16(value),
            BLDALPHA => self.alpha_coefficients = blending::AlphaCoefficients::from_u16(value),
            BLDY => self.brightness_coeff = (value & 0x1F).min(16) as u8,
            _ => {}
        }
    }

    /// Get current display mode
    pub fn display_mode(&self) -> DisplayMode {
        match self.dispcnt & 0x7 {
            0 => DisplayMode::Mode0,
            1 => DisplayMode::Mode1,
            2 => DisplayMode::Mode2,
            3 => DisplayMode::Mode3,
            4 => DisplayMode::Mode4,
            5 => DisplayMode::Mode5,
            _ => DisplayMode::Mode0,
        }
    }

    /// Execute PPU cycles
    pub fn step(&mut self, cycles: u32, vram: &[u8]) {
        self.cycles += cycles;

        while self.cycles >= CYCLES_PER_SCANLINE {
            self.cycles -= CYCLES_PER_SCANLINE;

            // Render scanline if visible
            if self.scanline < VISIBLE_SCANLINES {
                self.render_scanline(vram);
            }

            self.scanline += 1;

            if self.scanline >= SCANLINES_TOTAL {
                self.scanline = 0;
            }

            self.update_dispstat();
        }
    }

    /// Update DISPSTAT flags
    fn update_dispstat(&mut self) {
        if self.in_vblank() {
            self.dispstat |= 0x0001;
        } else {
            self.dispstat &= !0x0001;
        }
    }

    /// Check if in VBlank
    pub fn in_vblank(&self) -> bool {
        self.scanline >= VISIBLE_SCANLINES
    }

    /// Render a single scanline
    fn render_scanline(&mut self, vram: &[u8]) {
        match self.display_mode() {
            DisplayMode::Mode0 => {
                mode0::render_mode0_scanline(
                    self.scanline as usize,
                    SCREEN_WIDTH,
                    self.dispcnt,
                    &self.bg_control,
                    &self.bg_hofs,
                    &self.bg_vofs,
                    vram,
                    &self.palette_ram,
                    &mut self.framebuffer,
                );
            }
            DisplayMode::Mode3 => {
                mode3::render_mode3_scanline(self.scanline, vram, &mut self.framebuffer);
            }
            DisplayMode::Mode4 => {
                // Bit 4 of DISPCNT = frame select (0 or 1)
                let frame_select = (self.dispcnt & (1 << 4)) != 0;
                mode4::render_mode4_scanline(
                    &mut self.framebuffer,
                    vram,
                    &self.palette_ram,
                    self.scanline as usize,
                    frame_select,
                );
            }
            DisplayMode::Mode5 => {
                // Bit 4 of DISPCNT = frame select (0 or 1)
                let frame_select = (self.dispcnt & (1 << 4)) != 0;
                mode5::render_mode5_scanline(
                    &mut self.framebuffer,
                    vram,
                    self.scanline as usize,
                    frame_select,
                );
            }
            _ => {
                // TODO: Mode 1, 2 (affine backgrounds)
            }
        }

        // Render sprites if enabled (bit 12 of DISPCNT)
        if (self.dispcnt & (1 << 12)) != 0 {
            sprites::render_sprites_scanline(
                self.scanline as usize,
                SCREEN_WIDTH,
                &self.oam,
                vram,
                &self.palette_ram,
                &mut self.framebuffer,
            );
        }
    }

    /// Read byte from palette RAM
    pub fn read_palette_byte(&self, offset: usize) -> u8 {
        if offset < PALETTE_RAM_SIZE {
            self.palette_ram[offset]
        } else {
            0
        }
    }

    /// Write byte to palette RAM
    pub fn write_palette_byte(&mut self, offset: usize, value: u8) {
        if offset < PALETTE_RAM_SIZE {
            self.palette_ram[offset] = value;
        }
    }

    /// Read halfword from palette RAM
    pub fn read_palette_halfword(&self, offset: usize) -> u16 {
        if offset + 1 < PALETTE_RAM_SIZE {
            (self.palette_ram[offset] as u16) | ((self.palette_ram[offset + 1] as u16) << 8)
        } else {
            0
        }
    }

    /// Write halfword to palette RAM
    pub fn write_palette_halfword(&mut self, offset: usize, value: u16) {
        if offset + 1 < PALETTE_RAM_SIZE {
            self.palette_ram[offset] = (value & 0xFF) as u8;
            self.palette_ram[offset + 1] = ((value >> 8) & 0xFF) as u8;
        }
    }

    /// Read byte from OAM
    pub fn read_oam_byte(&self, offset: usize) -> u8 {
        if offset < OAM_SIZE {
            self.oam[offset]
        } else {
            0
        }
    }

    /// Write byte to OAM
    pub fn write_oam_byte(&mut self, offset: usize, value: u8) {
        if offset < OAM_SIZE {
            self.oam[offset] = value;
        }
    }

    /// Read halfword from OAM
    pub fn read_oam_halfword(&self, offset: usize) -> u16 {
        if offset + 1 < OAM_SIZE {
            (self.oam[offset] as u16) | ((self.oam[offset + 1] as u16) << 8)
        } else {
            0
        }
    }

    /// Write halfword to OAM
    pub fn write_oam_halfword(&mut self, offset: usize, value: u16) {
        if offset + 1 < OAM_SIZE {
            self.oam[offset] = (value & 0xFF) as u8;
            self.oam[offset + 1] = ((value >> 8) & 0xFF) as u8;
        }
    }

    /// Read sprite from OAM (index 0-127)
    pub fn read_sprite(&self, index: usize) -> SpriteAttribute {
        if index < OAM_SPRITE_COUNT {
            let offset = index * 8;
            SpriteAttribute::from_oam_bytes(&self.oam[offset..offset + 6])
        } else {
            SpriteAttribute::default()
        }
    }

    /// Get framebuffer for rendering
    pub fn framebuffer(&self) -> &[u16] {
        &self.framebuffer
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}
