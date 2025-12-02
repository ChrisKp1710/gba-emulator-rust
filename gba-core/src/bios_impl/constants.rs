/// BIOS - Software Interrupt (SWI) calls
/// GBA BIOS provides utility functions via SWI instruction
/// SWI Function Numbers
pub const SWI_SOFT_RESET: u8 = 0x00;
pub const SWI_REGISTER_RAM_RESET: u8 = 0x01;
pub const SWI_HALT: u8 = 0x02;
pub const SWI_STOP: u8 = 0x03;
pub const SWI_INTR_WAIT: u8 = 0x04;
pub const SWI_VBLANK_INTR_WAIT: u8 = 0x05;
pub const SWI_DIV: u8 = 0x06;
pub const SWI_DIV_ARM: u8 = 0x07;
pub const SWI_SQRT: u8 = 0x08;
pub const SWI_ARCTAN: u8 = 0x09;
pub const SWI_ARCTAN2: u8 = 0x0A;
pub const SWI_CPU_SET: u8 = 0x0B;
pub const SWI_CPU_FAST_SET: u8 = 0x0C;
pub const SWI_BG_AFFINE_SET: u8 = 0x0E;
pub const SWI_OBJ_AFFINE_SET: u8 = 0x0F;
pub const SWI_BIT_UNPACK: u8 = 0x10;
pub const SWI_LZ77_UNCOMP_WRAM: u8 = 0x11;
pub const SWI_LZ77_UNCOMP_VRAM: u8 = 0x12;
pub const SWI_HUFF_UNCOMP: u8 = 0x13;
pub const SWI_RL_UNCOMP_WRAM: u8 = 0x14;
pub const SWI_RL_UNCOMP_VRAM: u8 = 0x15;
pub const SWI_DIFF_8BIT_UNCOMP_WRAM: u8 = 0x16;
pub const SWI_DIFF_8BIT_UNCOMP_VRAM: u8 = 0x17;
pub const SWI_DIFF_16BIT_UNCOMP: u8 = 0x18;
pub const SWI_SOUND_BIAS: u8 = 0x19;
pub const SWI_SOUND_DRIVER_INIT: u8 = 0x1A;
pub const SWI_SOUND_DRIVER_MODE: u8 = 0x1B;
pub const SWI_SOUND_DRIVER_MAIN: u8 = 0x1C;
pub const SWI_SOUND_DRIVER_VSYNC: u8 = 0x1D;
pub const SWI_SOUND_CHANNEL_CLEAR: u8 = 0x1E;
pub const SWI_MIDI_KEY2FREQ: u8 = 0x1F;
pub const SWI_SOUND_DRIVER_VSYNC_OFF: u8 = 0x28;
pub const SWI_SOUND_DRIVER_VSYNC_ON: u8 = 0x29;

/// CPU Set control flags
pub const CPUSET_FILL: u32 = 1 << 24;  // Fill mode (vs copy)
pub const CPUSET_32BIT: u32 = 1 << 26; // 32-bit transfer (vs 16-bit)
