/// Save System - EEPROM Implementation
/// Serial EEPROM (512 bytes or 8 KB)
use super::types::SaveType;

/// EEPROM uses a serial protocol with DMA
/// Simplified implementation for basic functionality
pub struct Eeprom {
    data: Vec<u8>,
    size: usize,
    address_bits: u32, // 6 bits for 512B, 14 bits for 8KB
    
    // Serial state
    buffer: u64,
    bit_count: u32,
    reading: bool,
    writing: bool,
}

impl Eeprom {
    pub fn new(save_type: SaveType) -> Self {
        let size = save_type.size();
        let address_bits = match save_type {
            SaveType::Eeprom512B => 6,  // 64 addresses * 8 bytes = 512 bytes
            SaveType::Eeprom8K => 14,   // 1024 addresses * 8 bytes = 8 KB
            _ => 6,
        };

        Self {
            data: vec![0xFF; size],
            size,
            address_bits,
            buffer: 0,
            bit_count: 0,
            reading: false,
            writing: false,
        }
    }

    /// Process a single bit (DMA-based serial communication)
    pub fn process_bit(&mut self, bit: bool) -> bool {
        // Shift bit into buffer
        self.buffer = (self.buffer << 1) | (bit as u64);
        self.bit_count += 1;

        if self.reading {
            // Reading mode: return next bit from read buffer
            let out_bit = (self.buffer >> 63) != 0;
            self.buffer <<= 1;
            
            if self.bit_count >= 68 { // 4 dummy + 64 data bits
                self.reading = false;
                self.bit_count = 0;
                self.buffer = 0;
            }
            
            out_bit
        } else if self.writing {
            // Writing mode: collect bits
            if self.bit_count >= (2 + self.address_bits + 64) {
                // Command (2) + Address + Data (64)
                self.perform_write();
                self.writing = false;
                self.bit_count = 0;
                self.buffer = 0;
            }
            true
        } else {
            // Check for command
            if self.bit_count >= (2 + self.address_bits) {
                let command = (self.buffer >> (self.address_bits + 62)) & 0x3;
                
                match command {
                    0b11 => {
                        // Read request
                        self.perform_read();
                        self.reading = true;
                        self.bit_count = 0;
                    }
                    0b10 => {
                        // Write request
                        self.writing = true;
                    }
                    _ => {
                        // Unknown command
                        self.bit_count = 0;
                        self.buffer = 0;
                    }
                }
            }
            true
        }
    }

    /// Perform read operation
    fn perform_read(&mut self) {
        let address = ((self.buffer >> 62) & ((1 << self.address_bits) - 1)) as usize;
        let byte_addr = address * 8;
        
        // Load 8 bytes (64 bits) into buffer
        let mut data = 0u64;
        for i in 0..8 {
            let byte_idx = byte_addr + i;
            if byte_idx < self.data.len() {
                data = (data << 8) | (self.data[byte_idx] as u64);
            } else {
                data = (data << 8) | 0xFF;
            }
        }
        
        self.buffer = data;
    }

    /// Perform write operation
    fn perform_write(&mut self) {
        let shift_amt = 64;
        // Extract address from upper bits (avoiding overflow)
        let address = if shift_amt < 64 {
            ((self.buffer >> shift_amt) & ((1 << self.address_bits) - 1)) as usize
        } else {
            0
        };
        let byte_addr = address * 8;
        
        // Extract 64 bits of data
        let data = self.buffer & 0xFFFFFFFFFFFFFFFF;
        
        // Write 8 bytes
        for i in 0..8 {
            let byte_idx = byte_addr + i;
            if byte_idx < self.data.len() {
                self.data[byte_idx] = ((data >> (56 - i * 8)) & 0xFF) as u8;
            }
        }
    }

    /// Get entire data for saving
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Load data from file
    pub fn load_data(&mut self, data: Vec<u8>) {
        if data.len() == self.size {
            self.data = data;
        } else {
            self.data = data;
            self.data.resize(self.size, 0xFF);
        }
    }

    /// Reset EEPROM state
    pub fn reset(&mut self) {
        self.buffer = 0;
        self.bit_count = 0;
        self.reading = false;
        self.writing = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eeprom_basic() {
        let eeprom = Eeprom::new(SaveType::Eeprom512B);
        assert_eq!(eeprom.size, 512);
        assert_eq!(eeprom.address_bits, 6);
    }

    #[test]
    fn test_eeprom_8k() {
        let eeprom = Eeprom::new(SaveType::Eeprom8K);
        assert_eq!(eeprom.size, 8192);
        assert_eq!(eeprom.address_bits, 14);
    }

    #[test]
    fn test_eeprom_default_value() {
        let eeprom = Eeprom::new(SaveType::Eeprom512B);
        assert_eq!(eeprom.data[0], 0xFF);
        assert_eq!(eeprom.data[511], 0xFF);
    }

    #[test]
    fn test_eeprom_load_data() {
        let mut eeprom = Eeprom::new(SaveType::Eeprom512B);
        let test_data = vec![0x42; 512];
        eeprom.load_data(test_data);

        assert_eq!(eeprom.data[0], 0x42);
        assert_eq!(eeprom.data[511], 0x42);
    }
}
