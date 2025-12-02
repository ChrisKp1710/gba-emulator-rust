/// Timer Control Register (TMxCNT_H)
#[derive(Debug, Clone, Copy, Default)]
pub struct TimerControl {
    pub prescaler: u8,    // Bits 0-1: Frequency (0=1, 1=64, 2=256, 3=1024)
    pub count_up: bool,   // Bit 2: Cascade/Count-up timing
    pub irq_enable: bool, // Bit 6: IRQ enable on overflow
    pub enabled: bool,    // Bit 7: Timer enable
}

impl TimerControl {
    pub fn from_u16(value: u16) -> Self {
        Self {
            prescaler: (value & 0x3) as u8,
            count_up: (value & (1 << 2)) != 0,
            irq_enable: (value & (1 << 6)) != 0,
            enabled: (value & (1 << 7)) != 0,
        }
    }

    pub fn to_u16(&self) -> u16 {
        (self.prescaler as u16)
            | ((self.count_up as u16) << 2)
            | ((self.irq_enable as u16) << 6)
            | ((self.enabled as u16) << 7)
    }

    /// Get prescaler value in CPU cycles
    pub fn get_prescaler_cycles(&self) -> u32 {
        match self.prescaler {
            0 => 1,
            1 => 64,
            2 => 256,
            3 => 1024,
            _ => 1,
        }
    }
}
