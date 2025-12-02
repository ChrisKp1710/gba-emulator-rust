// Square Wave Channel (Channel 1 e 2)

/// Square Wave Channel
#[derive(Debug)]
pub struct SquareChannel {
    /// Ha sweep? (true per CH1, false per CH2)
    has_sweep: bool,

    // === Registri ===
    sweep_reg: u16,     // SOUND1CNT_L (solo CH1)
    duty_envelope: u16, // SOUNDxCNT_L/H
    frequency: u16,     // SOUNDxCNT_X

    // === State ===
    enabled: bool,
    phase: u32,
    frequency_timer: u32,
    envelope_volume: u8,
    envelope_timer: u32,
    sweep_timer: u32,
    shadow_frequency: u32,
}

impl SquareChannel {
    pub fn new(has_sweep: bool) -> Self {
        Self {
            has_sweep,
            sweep_reg: 0,
            duty_envelope: 0,
            frequency: 0,
            enabled: false,
            phase: 0,
            frequency_timer: 0,
            envelope_volume: 0,
            envelope_timer: 0,
            sweep_timer: 0,
            shadow_frequency: 0,
        }
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        // CH1: 0x04000060-0x04000065
        // CH2: 0x04000068-0x0400006D
        let offset = addr & 0x0F;

        match offset {
            0x0 => self.sweep_reg as u8,
            0x1 => (self.sweep_reg >> 8) as u8,
            0x2 => self.duty_envelope as u8,
            0x3 => (self.duty_envelope >> 8) as u8,
            0x4 => self.frequency as u8,
            0x5 => (self.frequency >> 8) as u8,
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) {
        let offset = addr & 0x0F;

        match offset {
            0x0 => self.sweep_reg = (self.sweep_reg & 0xFF00) | value as u16,
            0x1 => self.sweep_reg = (self.sweep_reg & 0x00FF) | ((value as u16) << 8),
            0x2 => self.duty_envelope = (self.duty_envelope & 0xFF00) | value as u16,
            0x3 => self.duty_envelope = (self.duty_envelope & 0x00FF) | ((value as u16) << 8),
            0x4 => self.frequency = (self.frequency & 0xFF00) | value as u16,
            0x5 => {
                self.frequency = (self.frequency & 0x00FF) | ((value as u16) << 8);
                // Trigger se bit 7 (bit 15 della halfword)
                if value & 0x80 != 0 {
                    self.trigger();
                }
            }
            _ => {}
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;
        self.phase = 0;
        self.envelope_volume = (self.duty_envelope >> 12) as u8 & 0x0F;
        self.frequency_timer = 0;
        self.envelope_timer = 0;
        self.sweep_timer = 0;

        if self.has_sweep {
            self.shadow_frequency = (self.frequency & 0x7FF) as u32;
        }
    }

    /// Avanza il canale di un ciclo
    pub fn step(&mut self) {
        if self.enabled {
            // TODO: Implementare frequency timer, envelope, sweep
            // Per ora placeholder
        }
    }
    /// Genera un sample audio (-15 a +15)
    pub fn get_sample(&self) -> i8 {
        if !self.enabled {
            0
        } else {
            // Duty cycle: bit 6-7 di duty_envelope
            let duty = ((self.duty_envelope >> 6) & 0x03) as usize;
            let duty_patterns = [
                [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
                [1, 0, 0, 0, 0, 0, 0, 1], // 25%
                [1, 0, 0, 0, 0, 1, 1, 1], // 50%
                [0, 1, 1, 1, 1, 1, 1, 0], // 75%
            ];

            let pattern = duty_patterns[duty];
            let phase_index = (self.phase % 8) as usize;

            if pattern[phase_index] != 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_channel_creation() {
        let ch1 = SquareChannel::new(true);
        let ch2 = SquareChannel::new(false);

        assert!(ch1.has_sweep);
        assert!(!ch2.has_sweep);
        assert!(!ch1.is_enabled());
    }

    #[test]
    fn test_trigger() {
        let mut ch = SquareChannel::new(false);

        // Setup volume massimo
        ch.duty_envelope = 0xF000; // Volume 15

        // Trigger
        ch.write_byte(0x04000065, 0x80);

        assert!(ch.is_enabled());
        assert_eq!(ch.envelope_volume, 15);
    }

    #[test]
    fn test_duty_cycle() {
        let mut ch = SquareChannel::new(false);

        // Setup: volume 10, duty 50% (bit 6-7 = 2)
        ch.duty_envelope = 0xA080; // Volume 10, duty 50%
        ch.trigger();

        // 50% duty = pattern [1,0,0,0,0,1,1,1]
        ch.phase = 0;
        assert_eq!(ch.get_sample(), 10); // pattern[0] = 1

        ch.phase = 1;
        assert_eq!(ch.get_sample(), -10); // pattern[1] = 0
    }
}
