/// PPU - Picture Processing Unit del GBA
/// Risoluzione: 240x160 pixel
/// 5 modalità grafiche (Mode 0-5)
///
/// MODE 3: Bitmap 240x160, 16-bit color (RGB555)
/// - VRAM 0x06000000-0x06017FFF (96 KB, ma Mode 3 usa solo primi 75 KB)
/// - Ogni pixel = 2 byte (RGB555: 5 bit R, 5 bit G, 5 bit B)
/// - Più semplice da implementare ma usa più memoria

pub const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 160;

/// Registri I/O LCD
pub const DISPCNT: u32 = 0x04000000; // Display Control
pub const DISPSTAT: u32 = 0x04000004; // Display Status
pub const VCOUNT: u32 = 0x04000006; // Vertical Counter

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

pub struct PPU {
    /// Frame buffer (RGB555 format: xBBBBBGGGGGRRRRR)
    pub framebuffer: Vec<u16>,

    /// Display Control Register (DISPCNT)
    pub dispcnt: u16,

    /// Display Status Register (DISPSTAT)
    pub dispstat: u16,

    /// Scanline corrente (VCOUNT)
    pub scanline: u16,

    /// Cicli PPU accumulati
    pub cycles: u32,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            dispcnt: 0,
            dispstat: 0,
            scanline: 0,
            cycles: 0,
        }
    }

    /// Leggi registro I/O
    pub fn read_register(&self, addr: u32) -> u16 {
        match addr {
            DISPCNT => self.dispcnt,
            DISPSTAT => self.dispstat | ((self.in_vblank() as u16) << 0),
            VCOUNT => self.scanline,
            _ => 0,
        }
    }

    /// Scrivi registro I/O
    pub fn write_register(&mut self, addr: u32, value: u16) {
        match addr {
            DISPCNT => {
                self.dispcnt = value;
                // Bit 0-2: Mode
                // Bit 4: Display frame select (Mode 4/5)
                // Bit 8-12: BG0-BG3, OBJ enable
            }
            DISPSTAT => {
                // Bit 3-5 sono read-write (VCount setting)
                self.dispstat = (self.dispstat & 0x0007) | (value & 0xFFF8);
            }
            _ => {}
        }
    }

    /// Ottieni modalità display corrente
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

    /// Esegui cicli PPU
    pub fn step(&mut self, cycles: u32, vram: &[u8]) {
        self.cycles += cycles;

        // Un scanline = 1232 cicli (960 draw + 272 hblank)
        // 228 scanlines totali (160 visibili + 68 vblank)
        const CYCLES_PER_SCANLINE: u32 = 1232;
        const SCANLINES_TOTAL: u16 = 228;

        while self.cycles >= CYCLES_PER_SCANLINE {
            self.cycles -= CYCLES_PER_SCANLINE;

            // Renderizza scanline se visibile
            if self.scanline < 160 {
                self.render_scanline(vram);
            }

            self.scanline += 1;

            if self.scanline >= SCANLINES_TOTAL {
                self.scanline = 0;
            }

            // Aggiorna DISPSTAT flags
            self.update_dispstat();
        }
    }

    /// Aggiorna flag DISPSTAT
    fn update_dispstat(&mut self) {
        // Bit 0: VBlank flag
        if self.in_vblank() {
            self.dispstat |= 0x0001;
        } else {
            self.dispstat &= !0x0001;
        }

        // Bit 1: HBlank flag (sempre 0 per semplicità, TODO: implementare)
        // Bit 2: VCount flag (TODO: implementare confronto)
    }

    /// Verifica se siamo in VBlank
    pub fn in_vblank(&self) -> bool {
        self.scanline >= 160
    }

    /// Renderizza un singolo scanline
    fn render_scanline(&mut self, vram: &[u8]) {
        match self.display_mode() {
            DisplayMode::Mode3 => self.render_mode3_scanline(vram),
            DisplayMode::Mode4 => {
                // TODO: Mode 4 (paletted)
            }
            _ => {
                // TODO: Altri mode (tiled)
            }
        }
    }

    /// Renderizza scanline in Mode 3 (bitmap 16-bit)
    fn render_mode3_scanline(&mut self, vram: &[u8]) {
        // Mode 3: VRAM è array di u16 (RGB555)
        // Offset = scanline * width * 2 byte
        let line = self.scanline as usize;
        let offset = line * SCREEN_WIDTH * 2;

        // Copia scanline da VRAM a framebuffer
        for x in 0..SCREEN_WIDTH {
            let vram_idx = offset + x * 2;

            // Leggi pixel RGB555 (little endian)
            if vram_idx + 1 < vram.len() {
                let pixel = (vram[vram_idx] as u16) | ((vram[vram_idx + 1] as u16) << 8);
                self.framebuffer[line * SCREEN_WIDTH + x] = pixel;
            } else {
                // Fuori bounds, pixel nero
                self.framebuffer[line * SCREEN_WIDTH + x] = 0;
            }
        }
    }

    /// Ottieni framebuffer per rendering
    pub fn framebuffer(&self) -> &[u16] {
        &self.framebuffer
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}
