use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CartridgeError {
    #[error("Failed to load ROM: {0}")]
    LoadError(String),

    #[error("Invalid ROM size")]
    InvalidSize,

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Informazioni header ROM GBA
#[derive(Debug, Clone)]
pub struct RomHeader {
    pub title: String,
    pub game_code: String,
    pub maker_code: String,
    pub version: u8,
}

pub struct Cartridge {
    pub rom: Vec<u8>,
    pub header: RomHeader,
    pub rom_path: Option<PathBuf>,
}

impl Cartridge {
    /// Carica una ROM da file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, CartridgeError> {
        let rom = fs::read(path.as_ref())?;

        if rom.len() < 0xC0 {
            return Err(CartridgeError::InvalidSize);
        }

        let header = Self::parse_header(&rom)?;
        let rom_path = Some(path.as_ref().to_path_buf());

        Ok(Self {
            rom,
            header,
            rom_path,
        })
    }

    /// Parse dell'header ROM
    fn parse_header(rom: &[u8]) -> Result<RomHeader, CartridgeError> {
        // Title @ 0xA0-0xAB
        let title_bytes = &rom[0xA0..0xAC];
        let title = String::from_utf8_lossy(title_bytes)
            .trim_end_matches('\0')
            .to_string();

        // Game Code @ 0xAC-0xAF
        let game_code_bytes = &rom[0xAC..0xB0];
        let game_code = String::from_utf8_lossy(game_code_bytes).to_string();

        // Maker Code @ 0xB0-0xB1
        let maker_code_bytes = &rom[0xB0..0xB2];
        let maker_code = String::from_utf8_lossy(maker_code_bytes).to_string();

        // Version @ 0xBC
        let version = rom[0xBC];

        Ok(RomHeader {
            title,
            game_code,
            maker_code,
            version,
        })
    }
}
