/// Save System - Auto-detection
/// Detect save type from ROM data
use super::constants::*;
use super::types::SaveType;

/// Detect save type by scanning ROM for identification strings
pub fn detect_save_type(rom: &[u8]) -> SaveType {
    // Convert ROM to string for searching (safe for ASCII strings)
    let rom_str = String::from_utf8_lossy(rom);

    // Check for save type strings in order of specificity
    // More specific strings first to avoid false positives

    if rom_str.contains(SAVE_FLASH1M_V) {
        return SaveType::Flash128K;
    }

    if rom_str.contains(SAVE_FLASH512_V) {
        return SaveType::Flash64K;
    }

    if rom_str.contains(SAVE_FLASH_V) {
        // Generic FLASH - default to 64K
        return SaveType::Flash64K;
    }

    if rom_str.contains(SAVE_EEPROM_V) {
        // EEPROM - determine size by ROM size
        // Games > 16MB typically use 8KB EEPROM
        if rom.len() > 16 * 1024 * 1024 {
            return SaveType::Eeprom8K;
        } else {
            return SaveType::Eeprom512B;
        }
    }

    if rom_str.contains(SAVE_SRAM_V) {
        return SaveType::Sram;
    }

    // No save type detected
    SaveType::None
}

/// Verify save type by checking multiple heuristics
pub fn verify_save_type(_rom: &[u8], detected_type: SaveType) -> SaveType {
    // Additional verification can be done here
    // For now, trust the detection
    detected_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_sram() {
        let mut rom = vec![0u8; 1024];
        let marker = b"SRAM_V123";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        assert_eq!(detect_save_type(&rom), SaveType::Sram);
    }

    #[test]
    fn test_detect_flash_64k() {
        let mut rom = vec![0u8; 1024];
        let marker = b"FLASH512_V";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        assert_eq!(detect_save_type(&rom), SaveType::Flash64K);
    }

    #[test]
    fn test_detect_flash_128k() {
        let mut rom = vec![0u8; 1024];
        let marker = b"FLASH1M_V";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        assert_eq!(detect_save_type(&rom), SaveType::Flash128K);
    }

    #[test]
    fn test_detect_eeprom_512b() {
        let mut rom = vec![0u8; 8 * 1024 * 1024]; // 8 MB ROM
        let marker = b"EEPROM_V";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        assert_eq!(detect_save_type(&rom), SaveType::Eeprom512B);
    }

    #[test]
    fn test_detect_eeprom_8k() {
        let mut rom = vec![0u8; 32 * 1024 * 1024]; // 32 MB ROM
        let marker = b"EEPROM_V";
        rom[100..100 + marker.len()].copy_from_slice(marker);

        assert_eq!(detect_save_type(&rom), SaveType::Eeprom8K);
    }

    #[test]
    fn test_detect_none() {
        let rom = vec![0u8; 1024];
        assert_eq!(detect_save_type(&rom), SaveType::None);
    }
}
