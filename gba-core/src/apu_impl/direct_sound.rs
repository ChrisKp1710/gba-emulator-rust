// Direct Sound A/B (DMA Audio)

/// Direct Sound Channel (A o B)
#[derive(Debug)]
pub struct DirectSound {
    /// FIFO buffer 32-byte
    fifo: [i8; 32],
    read_pos: usize,
    write_pos: usize,
}

impl DirectSound {
    pub fn new() -> Self {
        Self {
            fifo: [0; 32],
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Scrivi un sample nel FIFO
    pub fn write_sample(&mut self, value: i8) {
        self.fifo[self.write_pos] = value;
        self.write_pos = (self.write_pos + 1) % 32;
    }

    /// Leggi un sample dal FIFO
    pub fn read_sample(&mut self) -> i8 {
        if self.read_pos == self.write_pos {
            0 // FIFO vuoto
        } else {
            let sample = self.fifo[self.read_pos];
            self.read_pos = (self.read_pos + 1) % 32;
            sample
        }
    }

    /// Resetta il FIFO
    pub fn reset_fifo(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
    }

    /// Verifica se FIFO ha spazio
    #[allow(dead_code)]
    pub fn has_space(&self) -> bool {
        let used = if self.write_pos >= self.read_pos {
            self.write_pos - self.read_pos
        } else {
            32 - (self.read_pos - self.write_pos)
        };
        used < 32
    }
}

impl Default for DirectSound {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_write_read() {
        let mut ds = DirectSound::new();

        // Scrivi 4 sample
        ds.write_sample(10);
        ds.write_sample(20);
        ds.write_sample(30);
        ds.write_sample(40);

        // Leggi
        assert_eq!(ds.read_sample(), 10);
        assert_eq!(ds.read_sample(), 20);
        assert_eq!(ds.read_sample(), 30);
        assert_eq!(ds.read_sample(), 40);

        // FIFO vuoto
        assert_eq!(ds.read_sample(), 0);
    }

    #[test]
    fn test_fifo_reset() {
        let mut ds = DirectSound::new();

        ds.write_sample(50);
        ds.reset_fifo();

        assert_eq!(ds.read_sample(), 0, "FIFO should be empty after reset");
    }

    #[test]
    fn test_fifo_wraparound() {
        let mut ds = DirectSound::new();

        // Scrivi 20 sample
        for i in 0..20 {
            ds.write_sample(i as i8);
        }

        // Leggi primi 10
        for i in 0..10 {
            assert_eq!(ds.read_sample(), i as i8);
        }

        // Scrivi altri 10 (dovrebbe wrappare)
        for i in 100..110 {
            ds.write_sample(i as i8);
        }

        // FIFO ha spazio (10 consumati + 10 nuovi = 20/32)
        assert!(ds.has_space());
    }
}
