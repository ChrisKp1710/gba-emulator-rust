/// DMA Control Register (DMAxCNT_H)
#[derive(Debug, Clone, Copy, Default)]
pub struct DmaControl {
    pub dest_control: u8,    // Bits 5-6: Destination address control
    pub source_control: u8,  // Bits 7-8: Source address control
    pub repeat: bool,        // Bit 9: Repeat mode
    pub transfer_32bit: bool, // Bit 10: Transfer type (0=16bit, 1=32bit)
    pub game_pak_drq: bool,  // Bit 11: Game Pak DRQ (DMA3 only)
    pub timing: u8,          // Bits 12-13: Start timing
    pub irq_enable: bool,    // Bit 14: IRQ upon end
    pub enabled: bool,       // Bit 15: DMA enable
}

impl DmaControl {
    pub fn from_u16(value: u16) -> Self {
        Self {
            dest_control: ((value >> 5) & 0x3) as u8,
            source_control: ((value >> 7) & 0x3) as u8,
            repeat: (value & (1 << 9)) != 0,
            transfer_32bit: (value & (1 << 10)) != 0,
            game_pak_drq: (value & (1 << 11)) != 0,
            timing: ((value >> 12) & 0x3) as u8,
            irq_enable: (value & (1 << 14)) != 0,
            enabled: (value & (1 << 15)) != 0,
        }
    }

    pub fn to_u16(&self) -> u16 {
        ((self.dest_control as u16) << 5)
            | ((self.source_control as u16) << 7)
            | ((self.repeat as u16) << 9)
            | ((self.transfer_32bit as u16) << 10)
            | ((self.game_pak_drq as u16) << 11)
            | ((self.timing as u16) << 12)
            | ((self.irq_enable as u16) << 14)
            | ((self.enabled as u16) << 15)
    }

    /// Get transfer size in bytes
    pub fn transfer_size(&self) -> u32 {
        if self.transfer_32bit { 4 } else { 2 }
    }
}

/// DMA timing trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaTiming {
    Immediate,
    VBlank,
    HBlank,
    Special, // Audio FIFO or Video Capture
}

impl DmaTiming {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => DmaTiming::Immediate,
            1 => DmaTiming::VBlank,
            2 => DmaTiming::HBlank,
            3 => DmaTiming::Special,
            _ => DmaTiming::Immediate,
        }
    }
}
