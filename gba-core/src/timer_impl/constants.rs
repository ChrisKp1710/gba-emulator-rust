/// Timer - Hardware Timing System
/// GBA has 4 independent timers (TM0-TM3)
/// Timer registers base addresses
pub const TM0CNT_L: u32 = 0x04000100; // Timer 0 Counter/Reload
pub const TM0CNT_H: u32 = 0x04000102; // Timer 0 Control
pub const TM1CNT_L: u32 = 0x04000104; // Timer 1 Counter/Reload
pub const TM1CNT_H: u32 = 0x04000106; // Timer 1 Control
pub const TM2CNT_L: u32 = 0x04000108; // Timer 2 Counter/Reload
pub const TM2CNT_H: u32 = 0x0400010A; // Timer 2 Control
pub const TM3CNT_L: u32 = 0x0400010C; // Timer 3 Counter/Reload
pub const TM3CNT_H: u32 = 0x0400010E; // Timer 3 Control

/// Prescaler frequencies (CPU cycles per timer tick)
pub const PRESCALER_1: u32 = 1;
pub const PRESCALER_64: u32 = 64;
pub const PRESCALER_256: u32 = 256;
pub const PRESCALER_1024: u32 = 1024;

/// Number of hardware timers
pub const TIMER_COUNT: usize = 4;
