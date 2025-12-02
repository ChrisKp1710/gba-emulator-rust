/// Save System Tests - Separated test module
use crate::save::*;
use std::fs;
use std::path::PathBuf;

// ========== SaveType Tests ==========

#[test]
fn test_save_type_size() {
    assert_eq!(SaveType::None.size(), 0);
    assert_eq!(SaveType::Sram.size(), 0x8000);
    assert_eq!(SaveType::Flash64K.size(), 0x10000);
    assert_eq!(SaveType::Flash128K.size(), 0x20000);
    assert_eq!(SaveType::Eeprom512B.size(), 0x200);
    assert_eq!(SaveType::Eeprom8K.size(), 0x2000);
}

#[test]
fn test_save_type_is_flash() {
    assert!(SaveType::Flash64K.is_flash());
    assert!(SaveType::Flash128K.is_flash());
    assert!(!SaveType::Sram.is_flash());
    assert!(!SaveType::Eeprom512B.is_flash());
}

#[test]
fn test_save_type_is_eeprom() {
    assert!(SaveType::Eeprom512B.is_eeprom());
    assert!(SaveType::Eeprom8K.is_eeprom());
    assert!(!SaveType::Sram.is_eeprom());
    assert!(!SaveType::Flash64K.is_eeprom());
}

// ========== Detection Tests ==========

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

// ========== SRAM Tests ==========

#[test]
fn test_sram_read_write() {
    use crate::save_impl::sram::Sram;
    let mut sram = Sram::new(SaveType::Sram);

    sram.write_byte(0, 0x42);
    sram.write_byte(100, 0xAB);
    sram.write_byte(0x7FFF, 0xCD);

    assert_eq!(sram.read_byte(0), 0x42);
    assert_eq!(sram.read_byte(100), 0xAB);
    assert_eq!(sram.read_byte(0x7FFF), 0xCD);
}

#[test]
fn test_sram_default_value() {
    use crate::save_impl::sram::Sram;
    let sram = Sram::new(SaveType::Sram);

    assert_eq!(sram.read_byte(0), 0xFF);
    assert_eq!(sram.read_byte(500), 0xFF);
}

// ========== Flash Tests ==========

#[test]
fn test_flash_chip_id() {
    use crate::save_impl::flash::Flash;
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
}

#[test]
fn test_flash_write_byte() {
    use crate::save_impl::flash::Flash;
    let mut flash = Flash::new(SaveType::Flash64K);

    flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
    flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
    flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_BYTE);
    flash.write_byte(0x100, 0x42);

    assert_eq!(flash.read_byte(0x100), 0x42);
}

#[test]
fn test_flash_erase_sector() {
    use crate::save_impl::flash::Flash;
    let mut flash = Flash::new(SaveType::Flash64K);

    // Write data
    flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
    flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
    flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_BYTE);
    flash.write_byte(0, 0x42);

    // Erase sector
    flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_WRITE_ENABLE);
    flash.write_byte(FLASH_ADDR_CMD2, FLASH_CMD_WRITE_DISABLE);
    flash.write_byte(FLASH_ADDR_CMD1, FLASH_CMD_ERASE_SECTOR);
    flash.write_byte(0, 0x30);

    assert_eq!(flash.read_byte(0), 0xFF);
}

// ========== EEPROM Tests ==========

#[test]
fn test_eeprom_basic() {
    use crate::save_impl::eeprom::Eeprom;
    let eeprom = Eeprom::new(SaveType::Eeprom512B);
    assert_eq!(eeprom.data().len(), 512);
}

#[test]
fn test_eeprom_8k() {
    use crate::save_impl::eeprom::Eeprom;
    let eeprom = Eeprom::new(SaveType::Eeprom8K);
    assert_eq!(eeprom.data().len(), 8192);
}

// ========== SaveController Tests ==========

#[test]
fn test_save_controller_no_save() {
    let controller = SaveController::new();
    assert_eq!(controller.save_type(), SaveType::None);
    assert!(!controller.is_modified());
}

#[test]
fn test_save_controller_sram_detection() {
    let mut controller = SaveController::new();
    let mut rom = vec![0u8; 1024];
    let marker = b"SRAM_V123";
    rom[100..100 + marker.len()].copy_from_slice(marker);

    controller.init_from_rom(&rom, None);
    assert_eq!(controller.save_type(), SaveType::Sram);
}

#[test]
fn test_save_controller_sram_read_write() {
    let mut controller = SaveController::new();
    let mut rom = vec![0u8; 1024];
    let marker = b"SRAM_V";
    rom[100..100 + marker.len()].copy_from_slice(marker);

    controller.init_from_rom(&rom, None);

    controller.write_byte(0, 0x42);
    controller.write_byte(100, 0xAB);
    assert!(controller.is_modified());

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
    assert_eq!(controller.save_type(), SaveType::Flash128K);
}

// ========== File Persistence Tests ==========

#[test]
fn test_save_load_file() {
    let temp_dir = std::env::temp_dir();
    let save_path = temp_dir.join("test_save.sav");

    // Clean up any existing file
    let _ = fs::remove_file(&save_path);

    let mut controller = SaveController::new();
    let mut rom = vec![0u8; 1024];
    let marker = b"SRAM_V";
    rom[100..100 + marker.len()].copy_from_slice(marker);

    controller.init_from_rom(&rom, Some(PathBuf::from("test.gba")));

    // Write data
    controller.write_byte(0, 0x11);
    controller.write_byte(100, 0x22);
    controller.write_byte(1000, 0x33);

    // Save to file
    controller.save_to_file(&save_path).unwrap();

    // Create new controller and load
    let mut controller2 = SaveController::new();
    controller2.init_from_rom(&rom, Some(PathBuf::from("test.gba")));
    controller2.load_from_file(&save_path).unwrap();

    // Verify data
    assert_eq!(controller2.read_byte(0), 0x11);
    assert_eq!(controller2.read_byte(100), 0x22);
    assert_eq!(controller2.read_byte(1000), 0x33);

    // Clean up
    let _ = fs::remove_file(&save_path);
}

#[test]
fn test_auto_save() {
    let temp_dir = std::env::temp_dir();
    let save_path = temp_dir.join("test_autosave.sav");

    let _ = fs::remove_file(&save_path);

    let mut controller = SaveController::new();
    let mut rom = vec![0u8; 1024];
    let marker = b"SRAM_V";
    rom[100..100 + marker.len()].copy_from_slice(marker);

    controller.init_from_rom(&rom, Some(save_path.clone()));

    // Write data
    controller.write_byte(0, 0xAA);
    assert!(controller.is_modified());

    // Auto-save
    controller.auto_save().unwrap();
    assert!(!controller.is_modified());

    // Verify file exists
    assert!(save_path.exists());

    // Clean up
    let _ = fs::remove_file(&save_path);
}
