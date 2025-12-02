// Noise Channel (Channel 4)

/// Noise Channel con LFSR
#[derive(Debug)]
pub struct NoiseChannel {
    // === Registri ===
    length_envelope: u16, // SOUND4CNT_L
    frequency: u16,       // SOUND4CNT_H
    
    // === State ===
    enabled: bool,
    lfsr: u16, // Linear Feedback Shift Register
    frequency_timer: u32,
    envelope_volume: u8,
    envelope_timer: u32,
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            length_envelope: 0,
            frequency: 0,
            enabled: false,
            lfsr: 0x7FFF,
            frequency_timer: 0,
            envelope_volume: 0,
            envelope_timer: 0,
        }
    }
    
    pub fn read_byte(&self, addr: u32) -> u8 {
        // 0x04000078-0x0400007D
        let offset = addr & 0x0F;
        
        match offset {
            0x8 => self.length_envelope as u8,
            0x9 => (self.length_envelope >> 8) as u8,
            0xC => self.frequency as u8,
            0xD => (self.frequency >> 8) as u8,
            _ => 0,
        }
    }
    
    pub fn write_byte(&mut self, addr: u32, value: u8) {
        let offset = addr & 0x0F;
        
        match offset {
            0x8 => self.length_envelope = (self.length_envelope & 0xFF00) | value as u16,
            0x9 => self.length_envelope = (self.length_envelope & 0x00FF) | ((value as u16) << 8),
            0xC => self.frequency = (self.frequency & 0xFF00) | value as u16,
            0xD => {
                self.frequency = (self.frequency & 0x00FF) | ((value as u16) << 8);
                if value & 0x80 != 0 {
                    self.trigger();
                }
            }
            _ => {}
        }
    }
    
    fn trigger(&mut self) {
        self.enabled = true;
        self.lfsr = 0x7FFF;
        self.envelope_volume = (self.length_envelope >> 12) as u8 & 0x0F;
        self.frequency_timer = 0;
        self.envelope_timer = 0;
    }
    
    pub fn step(&mut self) {
        if self.enabled {
            // TODO: LFSR stepping e frequency timer
        }
    }
    
    /// Genera un sample noise
    pub fn get_sample(&self) -> i8 {
        if !self.enabled {
            0
        } else {
            // Output basato su bit 0 del LFSR
            if (self.lfsr & 1) != 0 {
                self.envelope_volume as i8
            } else {
                -(self.envelope_volume as i8)
            }
        }
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for NoiseChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_noise_creation() {
        let ch = NoiseChannel::new();
        assert!(!ch.is_enabled());
        assert_eq!(ch.lfsr, 0x7FFF);
    }
    
    #[test]
    fn test_trigger() {
        let mut ch = NoiseChannel::new();
        
        // Setup volume
        ch.length_envelope = 0xA000; // Volume 10
        
        // Trigger
        ch.write_byte(0x0400007D, 0x80);
        
        assert!(ch.is_enabled());
        assert_eq!(ch.envelope_volume, 10);
        assert_eq!(ch.lfsr, 0x7FFF);
    }
}
