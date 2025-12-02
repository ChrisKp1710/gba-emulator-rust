/// PPU - Picture Processing Unit del GBA
/// Risoluzione: 240x160 pixel
/// 5 modalità grafiche (Mode 0-5)

pub const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 160;

pub struct PPU {
    /// Frame buffer (RGB565 format)
    pub framebuffer: Vec<u16>,
    
    /// Scanline corrente
    pub scanline: u16,
    
    /// Cicli PPU
    pub cycles: u32,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            scanline: 0,
            cycles: 0,
        }
    }
    
    /// Esegui cicli PPU
    pub fn step(&mut self, cycles: u32) {
        self.cycles += cycles;
        
        // Un scanline = 1232 cicli
        // 228 scanlines totali (160 visibili + 68 vblank)
        const CYCLES_PER_SCANLINE: u32 = 1232;
        const SCANLINES_TOTAL: u16 = 228;
        
        if self.cycles >= CYCLES_PER_SCANLINE {
            self.cycles -= CYCLES_PER_SCANLINE;
            self.scanline += 1;
            
            if self.scanline >= SCANLINES_TOTAL {
                self.scanline = 0;
            }
        }
    }
    
    /// Verifica se siamo in VBlank
    pub fn in_vblank(&self) -> bool {
        self.scanline >= 160
    }
    
    /// Renderizza un singolo scanline
    pub fn render_scanline(&mut self) {
        // TODO: Implementazione rendering completo
        // Per ora, placeholder
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}
