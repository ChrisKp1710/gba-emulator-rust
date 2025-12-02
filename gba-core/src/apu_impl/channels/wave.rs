// Wave Output Channel (Channel 3)

/// Wave Output Channel con Wave RAM
#[derive(Debug)]
pub struct WaveChannel {
    // === Registri ===
    control: u16,       // SOUND3CNT_L
    length_volume: u16, // SOUND3CNT_H
    frequency: u16,     // SOUND3CNT_X

    /// Wave RAM - 32 sample * 4-bit (16 byte)
    wave_ram: [u8; 16],

    // === State ===
    enabled: bool,
    frequency_timer: u32,
    sample_index: usize,
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            control: 0,
            length_volume: 0,
            frequency: 0,
            wave_ram: [0; 16],
            enabled: false,
            frequency_timer: 0,
            sample_index: 0,
        }
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        // 0x04000070-0x04000075
        let offset = addr & 0x0F;

        match offset {
            0x0 => self.control as u8,
            0x1 => (self.control >> 8) as u8,
            0x2 => self.length_volume as u8,
            0x3 => (self.length_volume >> 8) as u8,
            0x4 => self.frequency as u8,
            0x5 => (self.frequency >> 8) as u8,
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) {
        let offset = addr & 0x0F;

        match offset {
            0x0 => self.control = (self.control & 0xFF00) | value as u16,
            0x1 => self.control = (self.control & 0x00FF) | ((value as u16) << 8),
            0x2 => self.length_volume = (self.length_volume & 0xFF00) | value as u16,
            0x3 => self.length_volume = (self.length_volume & 0x00FF) | ((value as u16) << 8),
            0x4 => self.frequency = (self.frequency & 0xFF00) | value as u16,
            0x5 => {
                self.frequency = (self.frequency & 0x00FF) | ((value as u16) << 8);
                if value & 0x80 != 0 {
                    self.trigger();
                }
            }
            _ => {}
        }
    }

    pub fn read_wave_ram(&self, addr: u32) -> u8 {
        let index = (addr - 0x04000090) as usize;
        if index < 16 {
            self.wave_ram[index]
        } else {
            0
        }
    }

    pub fn write_wave_ram(&mut self, addr: u32, value: u8) {
        let index = (addr - 0x04000090) as usize;
        if index < 16 {
            self.wave_ram[index] = value;
        }
    }

    fn trigger(&mut self) {
        // Bit 7 di control = channel enable
        let channel_enabled = (self.control >> 7) & 1 != 0;
        self.enabled = channel_enabled;
        self.sample_index = 0;
        self.frequency_timer = 0;
    }

    pub fn step(&mut self) {
        if self.enabled {
            // TODO: Frequency timer e sample playback
        }
    }

    /// Genera un sample audio
    pub fn get_sample(&self) -> i8 {
        if !self.enabled {
            0
        } else {
            // Leggi sample 4-bit da Wave RAM
            let byte_index = self.sample_index / 2;
            let nibble_high = self.sample_index.is_multiple_of(2);

            if byte_index >= 16 {
                0
            } else {
                let byte = self.wave_ram[byte_index];
                let sample_4bit = if nibble_high {
                    (byte >> 4) & 0x0F
                } else {
                    byte & 0x0F
                };

                // Volume control: bit 13-14 di length_volume
                let volume_code = (self.length_volume >> 13) & 0x03;
                let shift = match volume_code {
                    0 => 4, // Mute (shift right 4 = /16)
                    1 => 0, // 100%
                    2 => 1, // 50%
                    3 => 2, // 25%
                    _ => 0,
                };

                // Converti 4-bit (0-15) a signed (-8 a +7)
                let signed = (sample_4bit as i8) - 8;
                signed >> shift
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_ram_access() {
        let mut ch = WaveChannel::new();

        // Scrivi pattern in Wave RAM
        for i in 0..16 {
            ch.write_wave_ram(0x04000090 + i, (i as u8) * 0x11);
        }

        // Verifica lettura
        for i in 0..16 {
            assert_eq!(ch.read_wave_ram(0x04000090 + i), (i as u8) * 0x11);
        }
    }

    #[test]
    fn test_trigger() {
        let mut ch = WaveChannel::new();

        // Abilita channel (bit 7 di control)
        ch.control = 0x0080;

        // Trigger
        ch.write_byte(0x04000075, 0x80);

        assert!(ch.is_enabled());
    }
}
