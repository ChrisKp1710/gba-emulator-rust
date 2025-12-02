/// DMA - Direct Memory Access Controller
/// GBA has 4 DMA channels (DMA0-DMA3)
/// DMA0 registers (highest priority)
pub const DMA0SAD: u32 = 0x040000B0; // Source Address
pub const DMA0DAD: u32 = 0x040000B4; // Destination Address
pub const DMA0CNT_L: u32 = 0x040000B8; // Word Count
pub const DMA0CNT_H: u32 = 0x040000BA; // Control

/// DMA1 registers
pub const DMA1SAD: u32 = 0x040000BC;
pub const DMA1DAD: u32 = 0x040000C0;
pub const DMA1CNT_L: u32 = 0x040000C4;
pub const DMA1CNT_H: u32 = 0x040000C6;

/// DMA2 registers
pub const DMA2SAD: u32 = 0x040000C8;
pub const DMA2DAD: u32 = 0x040000CC;
pub const DMA2CNT_L: u32 = 0x040000D0;
pub const DMA2CNT_H: u32 = 0x040000D2;

/// DMA3 registers (lowest priority, most flexible)
pub const DMA3SAD: u32 = 0x040000D4;
pub const DMA3DAD: u32 = 0x040000D8;
pub const DMA3CNT_L: u32 = 0x040000DC;
pub const DMA3CNT_H: u32 = 0x040000DE;

/// Number of DMA channels
pub const DMA_CHANNEL_COUNT: usize = 4;

/// DMA timing modes
pub const TIMING_IMMEDIATE: u8 = 0;
pub const TIMING_VBLANK: u8 = 1;
pub const TIMING_HBLANK: u8 = 2;
pub const TIMING_SPECIAL: u8 = 3; // Special (audio FIFO, video capture)

/// Address control modes
pub const ADDR_INCREMENT: u8 = 0;
pub const ADDR_DECREMENT: u8 = 1;
pub const ADDR_FIXED: u8 = 2;
pub const ADDR_RELOAD: u8 = 3; // Increment and reload
