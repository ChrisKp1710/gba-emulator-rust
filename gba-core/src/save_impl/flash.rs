/// Save System - Flash Memory Implementation
/// 64 KB or 128 KB Flash with sector erase
use super::constants::*;
use super::types::{FlashState, SaveType};

pub struct Flash {
    data: Vec<u8>,
    size: usize,
    state: FlashState,
    bank: u8,          // Current bank (0 or 1 for 128K)
    chip_id: u16,      // Chip identification
    write_enable: bool,
}

impl Flash {
    pub fn new(save_type: SaveType) -> Self {
        let size = save_type.size();
        let chip_id = match save_type {
            SaveType::Flash64K => FLASH_MACRONIX_64K,
            SaveType::Flash128K => FLASH_MACRONIX_128K,
            _ => 0,
        };

        Self {
            data: vec![0xFF; size],
            size,
            state: FlashState::Ready,
            bank: 0,
            chip_id,
            write_enable: false,
        }
    }

    /// Read byte from Flash
    pub fn read_byte(&self, offset: u32) -> u8 {
        match self.state {
            FlashState::ChipId => {
                // Return chip ID bytes
                match offset & 0x1 {
                    0 => (self.chip_id & 0xFF) as u8,
                    1 => (self.chip_id >> 8) as u8,
                    _ => 0xFF,
                }
            }
            _ => {
                // Normal read - apply bank offset for 128K
                let bank_offset = if self.size > FLASH_64K_SIZE {
                    (self.bank as usize) * FLASH_64K_SIZE
                } else {
                    0
                };
                let addr = (offset as usize) + bank_offset;
                self.data.get(addr & (self.size - 1)).copied().unwrap_or(0xFF)
            }
        }
    }

    /// Write byte to Flash (command or data)
    pub fn write_byte(&mut self, offset: u32, value: u8) {
        match self.state {
            FlashState::Ready => {
                // Check for command sequence start
                if offset == FLASH_ADDR_CMD1 && value == FLASH_CMD_WRITE_ENABLE {
                    self.state = FlashState::Command1;
                }
            }
            FlashState::Command1 => {
                if offset == FLASH_ADDR_CMD2 && value == FLASH_CMD_WRITE_DISABLE {
                    self.state = FlashState::Command2;
                } else {
                    self.state = FlashState::Ready;
                }
            }
            FlashState::Command2 => {
                // Execute command
                match value {
                    FLASH_CMD_ENTER_ID => {
                        self.state = FlashState::ChipId;
                    }
                    FLASH_CMD_EXIT_ID => {
                        self.state = FlashState::Ready;
                    }
                    FLASH_CMD_ERASE_SECTOR => {
                        self.state = FlashState::Erase;
                    }
                    FLASH_CMD_ERASE_CHIP => {
                        // Erase entire chip
                        self.data.fill(0xFF);
                        self.state = FlashState::Ready;
                    }
                    FLASH_CMD_WRITE_BYTE => {
                        self.state = FlashState::Write;
                        self.write_enable = true;
                    }
                    FLASH_CMD_BANK_SWITCH if self.size > FLASH_64K_SIZE => {
                        self.state = FlashState::BankSwitch;
                    }
                    _ => {
                        self.state = FlashState::Ready;
                    }
                }
            }
            FlashState::Erase => {
                // Erase 4KB sector
                let sector = ((offset as usize) / FLASH_SECTOR_SIZE) * FLASH_SECTOR_SIZE;
                let bank_offset = if self.size > FLASH_64K_SIZE {
                    (self.bank as usize) * FLASH_64K_SIZE
                } else {
                    0
                };
                let start = (sector + bank_offset) & (self.size - 1);
                let end = (start + FLASH_SECTOR_SIZE).min(self.data.len());
                self.data[start..end].fill(0xFF);
                self.state = FlashState::Ready;
            }
            FlashState::Write => {
                // Write single byte
                if self.write_enable {
                    let bank_offset = if self.size > FLASH_64K_SIZE {
                        (self.bank as usize) * FLASH_64K_SIZE
                    } else {
                        0
                    };
                    let addr = ((offset as usize) + bank_offset) & (self.size - 1);
                    if addr < self.data.len() {
                        self.data[addr] = value;
                    }
                    self.write_enable = false;
                }
                self.state = FlashState::Ready;
            }
            FlashState::BankSwitch => {
                // Switch bank (0 or 1)
                self.bank = value & 1;
                self.state = FlashState::Ready;
            }
            FlashState::ChipId => {
                if value == FLASH_CMD_EXIT_ID {
                    self.state = FlashState::Ready;
                }
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

    /// Reset Flash state
    pub fn reset(&mut self) {
        self.state = FlashState::Ready;
        self.bank = 0;
        self.write_enable = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_chip_id() {
        let mut flash = Flash::new(SaveType::Flash64K);

        // Enter chip ID mode
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_ENTER_ID);

        // Read chip ID
        let id_low = flash.read_byte(0);
        let id_high = flash.read_byte(1);
        let chip_id = (id_high as u16) << 8 | id_low as u16;

        assert_eq!(chip_id, FLASH_MACRONIX_64K);

        // Exit chip ID mode
        flash.write_byte(0, FLASH_CMD_EXIT_ID);
    }

    #[test]
    fn test_flash_write_byte() {
        let mut flash = Flash::new(SaveType::Flash64K);

        // Write enable sequence + write byte command
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_BYTE);

        // Write data
        flash.write_byte(0x100, 0x42);

        // Read back
        assert_eq!(flash.read_byte(0x100), 0x42);
    }

    #[test]
    fn test_flash_erase_sector() {
        let mut flash = Flash::new(SaveType::Flash64K);

        // Write some data
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_BYTE);
        flash.write_byte(0, 0x42);

        // Erase sector
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_ERASE_SECTOR);
        flash.write_byte(0, 0x30);

        // Should be erased (0xFF)
        assert_eq!(flash.read_byte(0), 0xFF);
    }

    #[test]
    fn test_flash_bank_switch() {
        let mut flash = Flash::new(SaveType::Flash128K);

        // Write to bank 0
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_BYTE);
        flash.write_byte(0, 0x11);

        // Switch to bank 1
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_BANK_SWITCH);
        flash.write_byte(0, 1);

        // Write to bank 1
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_BYTE);
        flash.write_byte(0, 0x22);

        // Read from bank 1
        assert_eq!(flash.read_byte(0), 0x22);

        // Switch back to bank 0
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
        flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
        flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_BANK_SWITCH);
        flash.write_byte(0, 0);

        // Read from bank 0
        assert_eq!(flash.read_byte(0), 0x11);
    }
}
