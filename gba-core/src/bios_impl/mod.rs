/// BIOS - Software Interrupt Handler
/// Modular implementation
mod calls;
mod constants;

pub use calls::*;
pub use constants::*;

/// BIOS state and handler
pub struct Bios {
    // BIOS state (if needed for stateful operations)
    pub halted: bool,
    pub waiting_for_interrupt: bool,
}

impl Bios {
    pub fn new() -> Self {
        Self {
            halted: false,
            waiting_for_interrupt: false,
        }
    }

    /// Reset BIOS state
    pub fn reset(&mut self) {
        self.halted = false;
        self.waiting_for_interrupt = false;
    }

    /// Handle SWI call
    /// Returns tuple: (should_halt, should_wait_interrupt)
    pub fn handle_swi(&mut self, swi_number: u8) -> (bool, bool) {
        match swi_number {
            SWI_SOFT_RESET => {
                calls::soft_reset();
                (false, false)
            }
            SWI_HALT => {
                self.halted = true;
                (true, false)
            }
            SWI_STOP => {
                self.halted = true;
                (true, false)
            }
            SWI_INTR_WAIT | SWI_VBLANK_INTR_WAIT => {
                self.waiting_for_interrupt = true;
                (false, true)
            }
            // Math operations handled by CPU directly reading registers
            SWI_DIV | SWI_DIV_ARM | SWI_SQRT | SWI_ARCTAN | SWI_ARCTAN2 => (false, false),
            // Memory operations - handled by CPU with memory callbacks
            SWI_CPU_SET | SWI_CPU_FAST_SET => (false, false),
            // Decompression - handled by CPU with memory callbacks
            SWI_BIT_UNPACK | SWI_LZ77_UNCOMP_WRAM | SWI_LZ77_UNCOMP_VRAM | SWI_RL_UNCOMP_WRAM
            | SWI_RL_UNCOMP_VRAM => (false, false),
            // Sound driver - stub for now
            SWI_SOUND_BIAS
            | SWI_SOUND_DRIVER_INIT
            | SWI_SOUND_DRIVER_MODE
            | SWI_SOUND_DRIVER_MAIN
            | SWI_SOUND_DRIVER_VSYNC
            | SWI_SOUND_CHANNEL_CLEAR
            | SWI_MIDI_KEY2FREQ
            | SWI_SOUND_DRIVER_VSYNC_OFF
            | SWI_SOUND_DRIVER_VSYNC_ON => (false, false),
            // Affine operations - stub
            SWI_BG_AFFINE_SET | SWI_OBJ_AFFINE_SET => (false, false),
            // Unknown SWI
            _ => (false, false),
        }
    }

    /// Clear halt state
    pub fn clear_halt(&mut self) {
        self.halted = false;
    }

    /// Clear interrupt wait
    pub fn clear_wait(&mut self) {
        self.waiting_for_interrupt = false;
    }

    /// Check if halted
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    /// Check if waiting for interrupt
    pub fn is_waiting(&self) -> bool {
        self.waiting_for_interrupt
    }
}

impl Default for Bios {
    fn default() -> Self {
        Self::new()
    }
}
