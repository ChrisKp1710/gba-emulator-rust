// Registri di controllo audio

/// Sound Control Registers
#[derive(Debug)]
pub struct SoundRegisters {
    /// SOUNDCNT_L (0x04000080) - DMG Sound Control/Mixing
    /// Bit 0-2: Sound 1-4 Right Volume (0-7)
    /// Bit 4-6: Sound 1-4 Left Volume (0-7)
    /// Bit 8-11: Sound 1-4 Right Enable (4 bits)
    /// Bit 12-15: Sound 1-4 Left Enable (4 bits)
    pub soundcnt_l: u16,
    
    /// SOUNDCNT_H (0x04000082) - Direct Sound Control/Mixing
    /// Bit 0-1: Sound 1-4 Volume (0=25%, 1=50%, 2=100%)
    /// Bit 2: Direct Sound A Volume (0=50%, 1=100%)
    /// Bit 3: Direct Sound B Volume (0=50%, 1=100%)
    /// Bit 8: Direct Sound A Right Enable
    /// Bit 9: Direct Sound A Left Enable
    /// Bit 10: Direct Sound A Timer Select
    /// Bit 11: Direct Sound A Reset FIFO
    /// Bit 12: Direct Sound B Right Enable
    /// Bit 13: Direct Sound B Left Enable
    /// Bit 14: Direct Sound B Timer Select
    /// Bit 15: Direct Sound B Reset FIFO
    pub soundcnt_h: u16,
    
    /// SOUNDCNT_X (0x04000084) - Sound on/off
    /// Bit 0-3: Sound 1-4 Status (read-only)
    /// Bit 7: Master Sound Enable
    soundcnt_x: u16,
    
    /// SOUNDBIAS (0x04000088) - Sound PWM Control
    /// Bit 0-9: Bias Level (default 0x200)
    /// Bit 14-15: Sampling Rate (0=32768Hz default)
    pub soundbias: u16,
}

impl SoundRegisters {
    pub fn new() -> Self {
        Self {
            soundcnt_l: 0,
            soundcnt_h: 0,
            soundcnt_x: 0,
            soundbias: 0x200, // Default bias
        }
    }
    
    pub fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            0x04000080 => self.soundcnt_l as u8,
            0x04000081 => (self.soundcnt_l >> 8) as u8,
            0x04000082 => self.soundcnt_h as u8,
            0x04000083 => (self.soundcnt_h >> 8) as u8,
            0x04000084 => self.soundcnt_x as u8,
            0x04000085 => (self.soundcnt_x >> 8) as u8,
            0x04000088 => self.soundbias as u8,
            0x04000089 => (self.soundbias >> 8) as u8,
            _ => 0,
        }
    }
    
    pub fn write_byte(&mut self, addr: u32, value: u8) {
        match addr {
            0x04000080 => self.soundcnt_l = (self.soundcnt_l & 0xFF00) | value as u16,
            0x04000081 => self.soundcnt_l = (self.soundcnt_l & 0x00FF) | ((value as u16) << 8),
            0x04000082 => self.soundcnt_h = (self.soundcnt_h & 0xFF00) | value as u16,
            0x04000083 => self.soundcnt_h = (self.soundcnt_h & 0x00FF) | ((value as u16) << 8),
            0x04000084 => {
                // Solo bit 7 scrivibile
                self.soundcnt_x = (self.soundcnt_x & 0xFF7F) | (value as u16 & 0x80);
            }
            0x04000085 => {} // Read-only
            0x04000088 => self.soundbias = (self.soundbias & 0xFF00) | value as u16,
            0x04000089 => self.soundbias = (self.soundbias & 0x00FF) | ((value as u16) << 8),
            _ => {}
        }
    }
    
    /// Verifica se master audio è abilitato
    pub fn is_master_enabled(&self) -> bool {
        (self.soundcnt_x & 0x80) != 0
    }
    
    /// Aggiorna status bit per un canale (0-3)
    #[allow(dead_code)]
    pub fn set_channel_status(&mut self, channel: u8, enabled: bool) {
        if channel < 4 {
            if enabled {
                self.soundcnt_x |= 1 << channel;
            } else {
                self.soundcnt_x &= !(1 << channel);
            }
        }
    }
    
    /// Ottiene il volume GB (0-7) per left/right
    pub fn get_gb_volume(&self) -> (u8, u8) {
        let right = (self.soundcnt_l & 0x07) as u8;
        let left = ((self.soundcnt_l >> 4) & 0x07) as u8;
        (left, right)
    }
    
    /// Verifica se un canale GB è abilitato su left/right
    pub fn is_gb_channel_enabled(&self, channel: u8) -> (bool, bool) {
        if channel >= 4 {
            return (false, false);
        }
        let right = (self.soundcnt_l >> (8 + channel)) & 1 != 0;
        let left = (self.soundcnt_l >> (12 + channel)) & 1 != 0;
        (left, right)
    }
}

impl Default for SoundRegisters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_master_enable() {
        let mut regs = SoundRegisters::new();
        assert!(!regs.is_master_enabled());
        
        regs.write_byte(0x04000084, 0x80);
        assert!(regs.is_master_enabled());
    }
    
    #[test]
    fn test_channel_status() {
        let mut regs = SoundRegisters::new();
        
        regs.set_channel_status(0, true);
        assert_eq!(regs.soundcnt_x & 0x01, 0x01);
        
        regs.set_channel_status(0, false);
        assert_eq!(regs.soundcnt_x & 0x01, 0x00);
    }
    
    #[test]
    fn test_gb_volume() {
        let mut regs = SoundRegisters::new();
        regs.soundcnt_l = 0x0077; // Volume 7 per entrambi
        
        let (left, right) = regs.get_gb_volume();
        assert_eq!(left, 7);
        assert_eq!(right, 7);
    }
}
