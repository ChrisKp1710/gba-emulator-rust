/// Save System - Main Module
/// Unified save system with file persistence
mod constants;
mod detection;
pub mod eeprom;
pub mod flash;
pub mod sram;
mod types;

pub use constants::*;
pub use detection::*;
pub use types::{SaveMetadata, SaveType};

use eeprom::Eeprom;
use flash::Flash;
use sram::Sram;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Main Save controller
pub struct SaveController {
    save_type: SaveType,
    metadata: SaveMetadata,
    
    // Save media (only one is active based on type)
    sram: Option<Sram>,
    flash: Option<Flash>,
    eeprom: Option<Eeprom>,
    
    // Modified flag for auto-save
    modified: bool,
}

impl SaveController {
    /// Create new save controller with detection
    pub fn new() -> Self {
        Self {
            save_type: SaveType::None,
            metadata: SaveMetadata::new(SaveType::None),
            sram: None,
            flash: None,
            eeprom: None,
            modified: false,
        }
    }

    /// Initialize with detected save type from ROM
    pub fn init_from_rom(&mut self, rom: &[u8], rom_path: Option<PathBuf>) {
        let save_type = detect_save_type(rom);
        self.save_type = save_type;
        self.metadata = SaveMetadata::new(save_type);
        self.metadata.rom_path = rom_path;
        self.metadata.generate_save_path();

        // Create appropriate save media
        match save_type {
            SaveType::Sram => {
                self.sram = Some(Sram::new(save_type));
            }
            SaveType::Flash64K | SaveType::Flash128K => {
                self.flash = Some(Flash::new(save_type));
            }
            SaveType::Eeprom512B | SaveType::Eeprom8K => {
                self.eeprom = Some(Eeprom::new(save_type));
            }
            SaveType::None => {}
        }

        // Try to load existing save file
        if let Some(save_path) = self.metadata.save_path.clone() {
            let _ = self.load_from_file(&save_path);
        }
    }

    /// Read byte from save memory
    pub fn read_byte(&self, addr: u32) -> u8 {
        match self.save_type {
            SaveType::Sram => {
                if let Some(sram) = &self.sram {
                    let offset = addr & 0xFFFF; // 64 KB range
                    return sram.read_byte(offset);
                }
            }
            SaveType::Flash64K | SaveType::Flash128K => {
                if let Some(flash) = &self.flash {
                    let offset = addr & 0x1FFFF; // 128 KB range
                    return flash.read_byte(offset);
                }
            }
            _ => {}
        }
        0xFF
    }

    /// Write byte to save memory
    pub fn write_byte(&mut self, addr: u32, value: u8) {
        match self.save_type {
            SaveType::Sram => {
                if let Some(sram) = &mut self.sram {
                    let offset = addr & 0xFFFF;
                    sram.write_byte(offset, value);
                    self.modified = true;
                }
            }
            SaveType::Flash64K | SaveType::Flash128K => {
                if let Some(flash) = &mut self.flash {
                    let offset = addr & 0x1FFFF;
                    flash.write_byte(offset, value);
                    self.modified = true;
                }
            }
            _ => {}
        }
    }

    /// Process EEPROM DMA bit (for EEPROM only)
    pub fn eeprom_process_bit(&mut self, bit: bool) -> bool {
        if let Some(eeprom) = &mut self.eeprom {
            self.modified = true;
            return eeprom.process_bit(bit);
        }
        true
    }

    /// Save to file
    pub fn save_to_file(&mut self, path: &Path) -> io::Result<()> {
        let data = match self.save_type {
            SaveType::Sram => {
                self.sram.as_ref().map(|s| s.data())
            }
            SaveType::Flash64K | SaveType::Flash128K => {
                self.flash.as_ref().map(|f| f.data())
            }
            SaveType::Eeprom512B | SaveType::Eeprom8K => {
                self.eeprom.as_ref().map(|e| e.data())
            }
            SaveType::None => None,
        };

        if let Some(data) = data {
            fs::write(path, data)?;
            self.modified = false;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "No save data to write",
            ))
        }
    }

    /// Load from file
    pub fn load_from_file(&mut self, path: &Path) -> io::Result<()> {
        if !path.exists() {
            return Ok(()); // No save file yet - not an error
        }

        let data = fs::read(path)?;

        match self.save_type {
            SaveType::Sram => {
                if let Some(sram) = &mut self.sram {
                    sram.load_data(data);
                }
            }
            SaveType::Flash64K | SaveType::Flash128K => {
                if let Some(flash) = &mut self.flash {
                    flash.load_data(data);
                }
            }
            SaveType::Eeprom512B | SaveType::Eeprom8K => {
                if let Some(eeprom) = &mut self.eeprom {
                    eeprom.load_data(data);
                }
            }
            SaveType::None => {}
        }

        self.modified = false;
        Ok(())
    }

    /// Auto-save if modified
    pub fn auto_save(&mut self) -> io::Result<()> {
        if self.modified {
            if let Some(save_path) = self.metadata.save_path.clone() {
                return self.save_to_file(&save_path);
            }
        }
        Ok(())
    }

    /// Check if save is modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get save type
    pub fn save_type(&self) -> SaveType {
        self.save_type
    }

    /// Get save path
    pub fn save_path(&self) -> Option<&Path> {
        self.metadata.save_path.as_deref()
    }
}

impl Default for SaveController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_controller_no_save() {
        let controller = SaveController::new();
        assert_eq!(controller.save_type, SaveType::None);
        assert!(!controller.is_modified());
    }

    #[test]
    fn test_save_controller_sram_detection() {
        let mut controller = SaveController::new();
        let mut rom = vec![0u8; 1024];
        let marker = b"SRAM_V123";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        controller.init_from_rom(&rom, None);
        assert_eq!(controller.save_type, SaveType::Sram);
    }

    #[test]
    fn test_save_controller_sram_read_write() {
        let mut controller = SaveController::new();
        let mut rom = vec![0u8; 1024];
        let marker = b"SRAM_V";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        controller.init_from_rom(&rom, None);

        // Write
        controller.write_byte(0, 0x42);
        controller.write_byte(100, 0xAB);
        assert!(controller.is_modified());

        // Read
        assert_eq!(controller.read_byte(0), 0x42);
        assert_eq!(controller.read_byte(100), 0xAB);
    }

    #[test]
    fn test_save_controller_flash_detection() {
        let mut controller = SaveController::new();
        let mut rom = vec![0u8; 1024];
        let marker = b"FLASH1M_V";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        controller.init_from_rom(&rom, None);
        assert_eq!(controller.save_type, SaveType::Flash128K);
    }
}
