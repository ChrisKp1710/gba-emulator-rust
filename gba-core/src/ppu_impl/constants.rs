/// PPU - Constants and Memory Map
/// Screen dimensions
pub const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 160;

/// LCD I/O Registers
pub const DISPCNT: u32 = 0x04000000; // Display Control
pub const DISPSTAT: u32 = 0x04000004; // Display Status
pub const VCOUNT: u32 = 0x04000006; // Vertical Counter

/// Background Control Registers (BGxCNT)
pub const BG0CNT: u32 = 0x04000008;
pub const BG1CNT: u32 = 0x0400000A;
pub const BG2CNT: u32 = 0x0400000C;
pub const BG3CNT: u32 = 0x0400000E;

/// Background Scroll Registers (BGxHOFS/BGxVOFS)
pub const BG0HOFS: u32 = 0x04000010;
pub const BG0VOFS: u32 = 0x04000012;
pub const BG1HOFS: u32 = 0x04000014;
pub const BG1VOFS: u32 = 0x04000016;
pub const BG2HOFS: u32 = 0x04000018;
pub const BG2VOFS: u32 = 0x0400001A;
pub const BG3HOFS: u32 = 0x0400001C;
pub const BG3VOFS: u32 = 0x0400001E;

/// Palette RAM: 0x05000000-0x050003FF (1KB)
pub const PALETTE_RAM_SIZE: usize = 0x400;
pub const BG_PALETTE_SIZE: usize = 0x200;
pub const OBJ_PALETTE_OFFSET: usize = 0x200;

/// OAM (Object Attribute Memory): 0x07000000-0x070003FF (1KB)
pub const OAM_SIZE: usize = 0x400;
pub const OAM_SPRITE_COUNT: usize = 128;

/// OBJ tiles in VRAM: 0x06010000-0x06017FFF (32KB in Mode 0-2)
pub const OBJ_TILE_BASE: usize = 0x10000;

/// Timing constants
pub const CYCLES_PER_SCANLINE: u32 = 1232;
pub const SCANLINES_TOTAL: u16 = 228;
pub const VISIBLE_SCANLINES: u16 = 160;
