use crate::input::InputController;
use crate::interrupt::InterruptController;
use crate::memory::Memory;
use crate::ppu::PPU;
use gba_arm7tdmi::cpu::MemoryBus;

/// Bus principale del sistema GBA
pub struct Bus {
    pub memory: Memory,
    pub ppu: PPU,
    pub interrupt: InterruptController,
    pub input: InputController,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            ppu: PPU::new(),
            interrupt: InterruptController::new(),
            input: InputController::new(),
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
        // I/O Registers: 0x04000000-0x040003FE
        if (0x04000000..0x04000400).contains(&addr) {
            return self.read_io_byte(addr);
        }
        self.memory.read_byte(addr)
    }

    fn read_halfword(&mut self, addr: u32) -> u16 {
        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            return self.read_io_halfword(addr);
        }
        self.memory.read_halfword(addr)
    }

    fn read_word(&mut self, addr: u32) -> u32 {
        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            let low = self.read_io_halfword(addr);
            let high = self.read_io_halfword(addr + 2);
            return (low as u32) | ((high as u32) << 16);
        }
        self.memory.read_word(addr)
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            self.write_io_byte(addr, value);
            return;
        }
        self.memory.write_byte(addr, value);
    }

    fn write_halfword(&mut self, addr: u32, value: u16) {
        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            self.write_io_halfword(addr, value);
            return;
        }
        self.memory.write_halfword(addr, value);
    }

    fn write_word(&mut self, addr: u32, value: u32) {
        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            self.write_io_halfword(addr, value as u16);
            self.write_io_halfword(addr + 2, (value >> 16) as u16);
            return;
        }
        self.memory.write_word(addr, value);
    }
}

impl Bus {
    /// Leggi I/O register (halfword)
    fn read_io_halfword(&mut self, addr: u32) -> u16 {
        match addr & !1 {
            // PPU registers
            0x04000000 => self.ppu.read_register(addr), // DISPCNT
            0x04000004 => self.ppu.read_register(addr), // DISPSTAT
            0x04000006 => self.ppu.read_register(addr), // VCOUNT

            // Interrupt registers
            0x04000200 => self.interrupt.ie,         // IE
            0x04000202 => self.interrupt.if_,        // IF
            0x04000208 => self.interrupt.ime as u16, // IME

            // Input
            0x04000130 => self.input.read_keyinput(), // KEYINPUT

            _ => {
                // Altri I/O non implementati
                0
            }
        }
    }

    /// Scrivi I/O register (halfword)
    fn write_io_halfword(&mut self, addr: u32, value: u16) {
        match addr & !1 {
            // PPU registers
            0x04000000 => self.ppu.write_register(addr, value), // DISPCNT
            0x04000004 => self.ppu.write_register(addr, value), // DISPSTAT

            // Interrupt registers
            0x04000200 => self.interrupt.ie = value,
            0x04000202 => self.interrupt.if_ = value,
            0x04000208 => self.interrupt.ime = (value & 0x01) != 0,

            _ => {
                // Altri I/O non implementati
            }
        }
    }

    /// Leggi I/O register (byte)
    fn read_io_byte(&mut self, addr: u32) -> u8 {
        let halfword = self.read_io_halfword(addr & !1);
        if addr & 1 == 0 {
            (halfword & 0xFF) as u8
        } else {
            (halfword >> 8) as u8
        }
    }

    /// Scrivi I/O register (byte)
    fn write_io_byte(&mut self, addr: u32, value: u8) {
        let aligned = addr & !1;
        let current = self.read_io_halfword(aligned);
        let new_value = if addr & 1 == 0 {
            (current & 0xFF00) | (value as u16)
        } else {
            (current & 0x00FF) | ((value as u16) << 8)
        };
        self.write_io_halfword(aligned, new_value);
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
