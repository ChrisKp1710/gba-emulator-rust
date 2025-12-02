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
        // OAM: 0x07000000-0x070003FF
        if (0x07000000..0x07000400).contains(&addr) {
            let offset = (addr - 0x07000000) as usize;
            return self.ppu.read_oam_byte(offset);
        }

        // Palette RAM: 0x05000000-0x050003FF
        if (0x05000000..0x05000400).contains(&addr) {
            let offset = (addr - 0x05000000) as usize;
            return self.ppu.read_palette_byte(offset);
        }

        // I/O Registers: 0x04000000-0x040003FE
        if (0x04000000..0x04000400).contains(&addr) {
            return self.read_io_byte(addr);
        }
        self.memory.read_byte(addr)
    }

    fn read_halfword(&mut self, addr: u32) -> u16 {
        // OAM
        if (0x07000000..0x07000400).contains(&addr) {
            let offset = (addr - 0x07000000) as usize;
            return self.ppu.read_oam_halfword(offset);
        }

        // Palette RAM
        if (0x05000000..0x05000400).contains(&addr) {
            let offset = (addr - 0x05000000) as usize;
            return self.ppu.read_palette_halfword(offset);
        }

        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            return self.read_io_halfword(addr);
        }
        self.memory.read_halfword(addr)
    }

    fn read_word(&mut self, addr: u32) -> u32 {
        // OAM
        if (0x07000000..0x07000400).contains(&addr) {
            let low = self.read_halfword(addr);
            let high = self.read_halfword(addr + 2);
            return (low as u32) | ((high as u32) << 16);
        }

        // Palette RAM
        if (0x05000000..0x05000400).contains(&addr) {
            let low = self.read_halfword(addr);
            let high = self.read_halfword(addr + 2);
            return (low as u32) | ((high as u32) << 16);
        }

        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            let low = self.read_io_halfword(addr);
            let high = self.read_io_halfword(addr + 2);
            return (low as u32) | ((high as u32) << 16);
        }
        self.memory.read_word(addr)
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        // OAM
        if (0x07000000..0x07000400).contains(&addr) {
            let offset = (addr - 0x07000000) as usize;
            self.ppu.write_oam_byte(offset, value);
            return;
        }

        // Palette RAM
        if (0x05000000..0x05000400).contains(&addr) {
            let offset = (addr - 0x05000000) as usize;
            self.ppu.write_palette_byte(offset, value);
            return;
        }

        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            self.write_io_byte(addr, value);
            return;
        }
        self.memory.write_byte(addr, value);
    }

    fn write_halfword(&mut self, addr: u32, value: u16) {
        // OAM
        if (0x07000000..0x07000400).contains(&addr) {
            let offset = (addr - 0x07000000) as usize;
            self.ppu.write_oam_halfword(offset, value);
            return;
        }

        // Palette RAM
        if (0x05000000..0x05000400).contains(&addr) {
            let offset = (addr - 0x05000000) as usize;
            self.ppu.write_palette_halfword(offset, value);
            return;
        }

        // I/O Registers
        if (0x04000000..0x04000400).contains(&addr) {
            self.write_io_halfword(addr, value);
            return;
        }
        self.memory.write_halfword(addr, value);
    }

    fn write_word(&mut self, addr: u32, value: u32) {
        // OAM
        if (0x07000000..0x07000400).contains(&addr) {
            self.write_halfword(addr, value as u16);
            self.write_halfword(addr + 2, (value >> 16) as u16);
            return;
        }

        // Palette RAM
        if (0x05000000..0x05000400).contains(&addr) {
            self.write_halfword(addr, value as u16);
            self.write_halfword(addr + 2, (value >> 16) as u16);
            return;
        }

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
            0x04000008 => self.ppu.read_register(addr), // BG0CNT
            0x0400000A => self.ppu.read_register(addr), // BG1CNT
            0x0400000C => self.ppu.read_register(addr), // BG2CNT
            0x0400000E => self.ppu.read_register(addr), // BG3CNT
            0x04000010 => self.ppu.read_register(addr), // BG0HOFS
            0x04000012 => self.ppu.read_register(addr), // BG0VOFS
            0x04000014 => self.ppu.read_register(addr), // BG1HOFS
            0x04000016 => self.ppu.read_register(addr), // BG1VOFS
            0x04000018 => self.ppu.read_register(addr), // BG2HOFS
            0x0400001A => self.ppu.read_register(addr), // BG2VOFS
            0x0400001C => self.ppu.read_register(addr), // BG3HOFS
            0x0400001E => self.ppu.read_register(addr), // BG3VOFS

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
            0x04000008 => self.ppu.write_register(addr, value), // BG0CNT
            0x0400000A => self.ppu.write_register(addr, value), // BG1CNT
            0x0400000C => self.ppu.write_register(addr, value), // BG2CNT
            0x0400000E => self.ppu.write_register(addr, value), // BG3CNT
            0x04000010 => self.ppu.write_register(addr, value), // BG0HOFS
            0x04000012 => self.ppu.write_register(addr, value), // BG0VOFS
            0x04000014 => self.ppu.write_register(addr, value), // BG1HOFS
            0x04000016 => self.ppu.write_register(addr, value), // BG1VOFS
            0x04000018 => self.ppu.write_register(addr, value), // BG2HOFS
            0x0400001A => self.ppu.write_register(addr, value), // BG2VOFS
            0x0400001C => self.ppu.write_register(addr, value), // BG3HOFS
            0x0400001E => self.ppu.write_register(addr, value), // BG3VOFS

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
