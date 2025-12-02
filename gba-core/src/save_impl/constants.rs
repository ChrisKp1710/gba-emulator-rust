/// Save System - Constants
/// Save types and memory regions
/// Save type detection strings
pub const SAVE_EEPROM_V: &str = "EEPROM_V";
pub const SAVE_SRAM_V: &str = "SRAM_V";
pub const SAVE_FLASH_V: &str = "FLASH_V";
pub const SAVE_FLASH512_V: &str = "FLASH512_V";
pub const SAVE_FLASH1M_V: &str = "FLASH1M_V";

/// Save memory regions
pub const SRAM_START: u32 = 0x0E000000;
pub const SRAM_END: u32 = 0x0E00FFFF;
pub const SRAM_SIZE: usize = 0x10000; // 64 KB max

pub const FLASH_START: u32 = 0x0E000000;
pub const FLASH_END: u32 = 0x0E01FFFF;
pub const FLASH_64K_SIZE: usize = 0x10000;  // 64 KB
pub const FLASH_128K_SIZE: usize = 0x20000; // 128 KB

pub const EEPROM_START: u32 = 0x0D000000;
pub const EEPROM_END: u32 = 0x0DFFFFFF;
pub const EEPROM_512B_SIZE: usize = 0x200;   // 512 bytes
pub const EEPROM_8K_SIZE: usize = 0x2000;    // 8 KB

/// Flash commands
pub const FLASH_CMD_READ: u8 = 0xFF;
pub const FLASH_CMD_WRITE_ENABLE: u8 = 0xAA;
pub const FLASH_CMD_WRITE_DISABLE: u8 = 0x55;
pub const FLASH_CMD_ERASE_SECTOR: u8 = 0x30;
pub const FLASH_CMD_ERASE_CHIP: u8 = 0x10;
pub const FLASH_CMD_WRITE_BYTE: u8 = 0xA0;
pub const FLASH_CMD_ENTER_ID: u8 = 0x90;
pub const FLASH_CMD_EXIT_ID: u8 = 0xF0;
pub const FLASH_CMD_BANK_SWITCH: u8 = 0xB0;

/// Flash addresses
pub const FLASH_ADDR_CMD1: u32 = 0x5555;
pub const FLASH_ADDR_CMD2: u32 = 0x2AAA;

/// Flash chip IDs (Macronix, Panasonic, Atmel, Sanyo)
pub const FLASH_MACRONIX_64K: u16 = 0x1CC2;
pub const FLASH_MACRONIX_128K: u16 = 0x09C2;
pub const FLASH_PANASONIC_64K: u16 = 0x1B32;
pub const FLASH_ATMEL_64K: u16 = 0x3D1F;
pub const FLASH_SANYO_128K: u16 = 0x1362;

/// Flash sector size (typically 4 KB)
pub const FLASH_SECTOR_SIZE: usize = 0x1000;
