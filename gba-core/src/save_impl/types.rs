/// Save System - Types
/// Save types and detection
use std::path::PathBuf;

/// Type of save memory used by the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveType {
    None,
    Sram,           // 32-64 KB, simple R/W
    Flash64K,       // 64 KB Flash
    Flash128K,      // 128 KB Flash
    Eeprom512B,     // 512 bytes EEPROM
    Eeprom8K,       // 8 KB EEPROM
}

impl SaveType {
    /// Get the size in bytes for this save type
    pub fn size(&self) -> usize {
        match self {
            SaveType::None => 0,
            SaveType::Sram => 0x8000,          // 32 KB (most common)
            SaveType::Flash64K => 0x10000,     // 64 KB
            SaveType::Flash128K => 0x20000,    // 128 KB
            SaveType::Eeprom512B => 0x200,     // 512 bytes
            SaveType::Eeprom8K => 0x2000,      // 8 KB
        }
    }

    /// Check if this is a Flash type
    pub fn is_flash(&self) -> bool {
        matches!(self, SaveType::Flash64K | SaveType::Flash128K)
    }

    /// Check if this is EEPROM
    pub fn is_eeprom(&self) -> bool {
        matches!(self, SaveType::Eeprom512B | SaveType::Eeprom8K)
    }

    /// Get file extension for save file
    pub fn extension(&self) -> &str {
        match self {
            SaveType::None => "",
            SaveType::Sram => "sav",
            SaveType::Flash64K | SaveType::Flash128K => "sav",
            SaveType::Eeprom512B | SaveType::Eeprom8K => "sav",
        }
    }
}

/// Flash state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashState {
    Ready,
    Command1,
    Command2,
    Erase,
    Write,
    ChipId,
    BankSwitch,
}

/// Save file metadata
#[derive(Debug, Clone)]
pub struct SaveMetadata {
    pub save_type: SaveType,
    pub rom_path: Option<PathBuf>,
    pub save_path: Option<PathBuf>,
    pub modified: bool,
}

impl SaveMetadata {
    pub fn new(save_type: SaveType) -> Self {
        Self {
            save_type,
            rom_path: None,
            save_path: None,
            modified: false,
        }
    }

    /// Generate save file path from ROM path
    pub fn generate_save_path(&mut self) {
        if let Some(rom_path) = &self.rom_path {
            let mut save_path = rom_path.clone();
            save_path.set_extension(self.save_type.extension());
            self.save_path = Some(save_path);
        }
    }
}
