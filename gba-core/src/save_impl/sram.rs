/// Save System - SRAM Implementation
/// Simple battery-backed SRAM (32-64 KB)
use super::types::SaveType;

pub struct Sram {
    data: Vec<u8>,
    size: usize,
}

impl Sram {
    pub fn new(save_type: SaveType) -> Self {
        let size = save_type.size();
        Self {
            data: vec![0xFF; size], // SRAM defaults to 0xFF when empty
            size,
        }
    }

    /// Read byte from SRAM
    pub fn read_byte(&self, offset: u32) -> u8 {
        let offset = (offset as usize) & (self.size - 1); // Wrap around
        self.data.get(offset).copied().unwrap_or(0xFF)
    }

    /// Write byte to SRAM
    pub fn write_byte(&mut self, offset: u32, value: u8) {
        let offset = (offset as usize) & (self.size - 1); // Wrap around
        if offset < self.data.len() {
            self.data[offset] = value;
        }
    }

    /// Get entire data for saving to file
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Load data from file
    pub fn load_data(&mut self, data: Vec<u8>) {
        if data.len() == self.size {
            self.data = data;
        } else {
            // Resize if needed
            self.data = data;
            self.data.resize(self.size, 0xFF);
        }
    }

    /// Clear all data (reset to 0xFF)
    pub fn clear(&mut self) {
        self.data.fill(0xFF);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sram_read_write() {
        let mut sram = Sram::new(SaveType::Sram);

        // Write some data
        sram.write_byte(0, 0x42);
        sram.write_byte(100, 0xAB);
        sram.write_byte(0x7FFF, 0xCD);

        // Read it back
        assert_eq!(sram.read_byte(0), 0x42);
        assert_eq!(sram.read_byte(100), 0xAB);
        assert_eq!(sram.read_byte(0x7FFF), 0xCD);
    }

    #[test]
    fn test_sram_wraparound() {
        let mut sram = Sram::new(SaveType::Sram);
        let size = sram.size as u32;

        // Write beyond size (should wrap)
        sram.write_byte(size, 0x11);
        assert_eq!(sram.read_byte(0), 0x11);

        sram.write_byte(size + 100, 0x22);
        assert_eq!(sram.read_byte(100), 0x22);
    }

    #[test]
    fn test_sram_default_value() {
        let sram = Sram::new(SaveType::Sram);

        // Unwritten locations should be 0xFF
        assert_eq!(sram.read_byte(0), 0xFF);
        assert_eq!(sram.read_byte(500), 0xFF);
    }

    #[test]
    fn test_sram_clear() {
        let mut sram = Sram::new(SaveType::Sram);

        sram.write_byte(0, 0x42);
        sram.write_byte(100, 0xAB);

        sram.clear();

        assert_eq!(sram.read_byte(0), 0xFF);
        assert_eq!(sram.read_byte(100), 0xFF);
    }

    #[test]
    fn test_sram_load_data() {
        let mut sram = Sram::new(SaveType::Sram);

        let test_data = vec![0x11; 0x8000];
        sram.load_data(test_data);

        assert_eq!(sram.read_byte(0), 0x11);
        assert_eq!(sram.read_byte(100), 0x11);
        assert_eq!(sram.read_byte(0x7FFF), 0x11);
    }
}
