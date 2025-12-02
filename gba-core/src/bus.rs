use crate::memory::Memory;
use crate::ppu::PPU;
use crate::interrupt::InterruptController;
use gba_arm7tdmi::cpu::MemoryBus;

/// Bus principale del sistema GBA
pub struct Bus {
    pub memory: Memory,
    pub ppu: PPU,
    pub interrupt: InterruptController,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            ppu: PPU::new(),
            interrupt: InterruptController::new(),
        }
    }
    
    pub fn load_bios(&mut self, bios: Vec<u8>) {
        self.memory.load_bios(bios);
    }
    
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(rom);
    }
}

impl MemoryBus for Bus {
    fn read_byte(&mut self, addr: u32) -> u8 {
        self.memory.read_byte(addr)
    }
    
    fn read_halfword(&mut self, addr: u32) -> u16 {
        self.memory.read_halfword(addr)
    }
    
    fn read_word(&mut self, addr: u32) -> u32 {
        self.memory.read_word(addr)
    }
    
    fn write_byte(&mut self, addr: u32, value: u8) {
        self.memory.write_byte(addr, value);
    }
    
    fn write_halfword(&mut self, addr: u32, value: u16) {
        self.memory.write_halfword(addr, value);
    }
    
    fn write_word(&mut self, addr: u32, value: u32) {
        self.memory.write_word(addr, value);
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
