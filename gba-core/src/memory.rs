//==============================================================================
// MEMORY MAPPING DEL GAME BOY ADVANCE
//==============================================================================
// Il GBA ha una mappa memoria molto specifica. Ogni regione ha caratteristiche
// diverse in termini di velocità, dimensione e scopo.
// 
// MAPPA COMPLETA:
// 0x00000000 - 0x00003FFF : BIOS (16 KB) - Sistema, READ-ONLY
// 0x02000000 - 0x0203FFFF : EWRAM (256 KB) - RAM esterna, lenta
// 0x03000000 - 0x03007FFF : IWRAM (32 KB) - RAM interna, VELOCE
// 0x04000000 - 0x040003FE : I/O Registers - Controllo hardware
// 0x05000000 - 0x050003FF : Palette RAM (1 KB) - Colori
// 0x06000000 - 0x06017FFF : VRAM (96 KB) - Grafica
// 0x07000000 - 0x070003FF : OAM (1 KB) - Sprite attributes
// 0x08000000 - 0x09FFFFFF : ROM (32 MB) - Gioco, READ-ONLY
// 0x0E000000 - 0x0E00FFFF : SRAM (64 KB) - Salvataggi
// 
// TIMING (cicli di attesa):
// - BIOS/IWRAM: 0 wait states (più veloce)
// - EWRAM: 2 wait states
// - ROM: 0-8 wait states (dipende dalla regione e configurazione)
// - SRAM: 8 wait states (più lento)
// 
// MIRRORS:
// Alcune regioni sono "mirrorate" (replicate) in più indirizzi.
// Es: ROM a 0x08000000 è visibile anche a 0x0A000000, 0x0C000000
//==============================================================================

/// Mappa della memoria del GBA con timing e caratteristiche

pub struct Memory {
    // BIOS - Sistema BIOS (16 KB)
    pub bios: Vec<u8>,
    
    // On-board Work RAM (256 KB)
    pub ewram: Vec<u8>,
    
    // On-chip Work RAM (32 KB) - Più veloce
    pub iwram: Vec<u8>,
    
    // I/O Registers
    pub io_registers: Vec<u8>,
    
    // Palette RAM (1 KB)
    pub palette_ram: Vec<u8>,
    
    // VRAM (96 KB)
    pub vram: Vec<u8>,
    
    // OAM - Object Attribute Memory (1 KB)
    pub oam: Vec<u8>,
    
    // Game ROM (caricata da cartridge)
    pub rom: Vec<u8>,
    
    // Save RAM
    pub sram: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bios: vec![0; 0x4000],           // 16 KB
            ewram: vec![0; 0x40000],         // 256 KB
            iwram: vec![0; 0x8000],          // 32 KB
            io_registers: vec![0; 0x400],    // 1 KB
            palette_ram: vec![0; 0x400],     // 1 KB
            vram: vec![0; 0x18000],          // 96 KB
            oam: vec![0; 0x400],             // 1 KB
            rom: Vec::new(),
            sram: vec![0; 0x10000],          // 64 KB max
        }
    }
    
    pub fn load_bios(&mut self, bios: Vec<u8>) {
        self.bios = bios;
    }
    
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }
    
    pub fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            // BIOS
            0x0000_0000..=0x0000_3FFF => {
                self.bios.get(addr as usize).copied().unwrap_or(0)
            }
            
            // External WRAM
            0x0200_0000..=0x0203_FFFF => {
                let offset = (addr - 0x0200_0000) as usize;
                self.ewram.get(offset).copied().unwrap_or(0)
            }
            
            // Internal WRAM
            0x0300_0000..=0x0300_7FFF => {
                let offset = (addr - 0x0300_0000) as usize;
                self.iwram.get(offset).copied().unwrap_or(0)
            }
            
            // I/O Registers
            0x0400_0000..=0x0400_03FF => {
                let offset = (addr - 0x0400_0000) as usize;
                self.io_registers.get(offset).copied().unwrap_or(0)
            }
            
            // Palette RAM
            0x0500_0000..=0x0500_03FF => {
                let offset = (addr - 0x0500_0000) as usize;
                self.palette_ram.get(offset).copied().unwrap_or(0)
            }
            
            // VRAM
            0x0600_0000..=0x0601_7FFF => {
                let offset = (addr - 0x0600_0000) as usize;
                self.vram.get(offset).copied().unwrap_or(0)
            }
            
            // OAM
            0x0700_0000..=0x0700_03FF => {
                let offset = (addr - 0x0700_0000) as usize;
                self.oam.get(offset).copied().unwrap_or(0)
            }
            
            // Game ROM (mirrors)
            0x0800_0000..=0x09FF_FFFF | 
            0x0A00_0000..=0x0BFF_FFFF | 
            0x0C00_0000..=0x0DFF_FFFF => {
                let offset = (addr & 0x01FF_FFFF) as usize;
                self.rom.get(offset).copied().unwrap_or(0xFF)
            }
            
            // SRAM
            0x0E00_0000..=0x0E00_FFFF => {
                let offset = (addr - 0x0E00_0000) as usize;
                self.sram.get(offset).copied().unwrap_or(0xFF)
            }
            
            _ => 0,
        }
    }
    
    pub fn read_halfword(&self, addr: u32) -> u16 {
        let low = self.read_byte(addr) as u16;
        let high = self.read_byte(addr + 1) as u16;
        (high << 8) | low
    }
    
    pub fn read_word(&self, addr: u32) -> u32 {
        let b0 = self.read_byte(addr) as u32;
        let b1 = self.read_byte(addr + 1) as u32;
        let b2 = self.read_byte(addr + 2) as u32;
        let b3 = self.read_byte(addr + 3) as u32;
        (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
    }
    
    pub fn write_byte(&mut self, addr: u32, value: u8) {
        match addr {
            // BIOS - read only
            0x0000_0000..=0x0000_3FFF => {}
            
            // External WRAM
            0x0200_0000..=0x0203_FFFF => {
                let offset = (addr - 0x0200_0000) as usize;
                if let Some(byte) = self.ewram.get_mut(offset) {
                    *byte = value;
                }
            }
            
            // Internal WRAM
            0x0300_0000..=0x0300_7FFF => {
                let offset = (addr - 0x0300_0000) as usize;
                if let Some(byte) = self.iwram.get_mut(offset) {
                    *byte = value;
                }
            }
            
            // I/O Registers
            0x0400_0000..=0x0400_03FF => {
                let offset = (addr - 0x0400_0000) as usize;
                if let Some(byte) = self.io_registers.get_mut(offset) {
                    *byte = value;
                }
            }
            
            // Palette RAM
            0x0500_0000..=0x0500_03FF => {
                let offset = (addr - 0x0500_0000) as usize;
                if let Some(byte) = self.palette_ram.get_mut(offset) {
                    *byte = value;
                }
            }
            
            // VRAM
            0x0600_0000..=0x0601_7FFF => {
                let offset = (addr - 0x0600_0000) as usize;
                if let Some(byte) = self.vram.get_mut(offset) {
                    *byte = value;
                }
            }
            
            // OAM
            0x0700_0000..=0x0700_03FF => {
                let offset = (addr - 0x0700_0000) as usize;
                if let Some(byte) = self.oam.get_mut(offset) {
                    *byte = value;
                }
            }
            
            // ROM - read only
            0x0800_0000..=0x0DFF_FFFF => {}
            
            // SRAM
            0x0E00_0000..=0x0E00_FFFF => {
                let offset = (addr - 0x0E00_0000) as usize;
                if let Some(byte) = self.sram.get_mut(offset) {
                    *byte = value;
                }
            }
            
            _ => {}
        }
    }
    
    pub fn write_halfword(&mut self, addr: u32, value: u16) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr + 1, ((value >> 8) & 0xFF) as u8);
    }
    
    pub fn write_word(&mut self, addr: u32, value: u32) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr + 1, ((value >> 8) & 0xFF) as u8);
        self.write_byte(addr + 2, ((value >> 16) & 0xFF) as u8);
        self.write_byte(addr + 3, ((value >> 24) & 0xFF) as u8);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
