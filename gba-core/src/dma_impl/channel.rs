use super::types::{DmaControl, DmaTiming};

/// Single DMA channel
#[derive(Debug, Clone)]
pub struct DmaChannel {
    pub channel_id: usize,
    pub source_addr: u32,
    pub dest_addr: u32,
    pub word_count: u16,
    pub control: DmaControl,
    
    // Internal state
    internal_source: u32,
    internal_dest: u32,
    internal_count: u16,
    pub active: bool,
}

impl DmaChannel {
    pub fn new(channel_id: usize) -> Self {
        Self {
            channel_id,
            source_addr: 0,
            dest_addr: 0,
            word_count: 0,
            control: DmaControl::default(),
            internal_source: 0,
            internal_dest: 0,
            internal_count: 0,
            active: false,
        }
    }

    /// Reset channel to initial state
    pub fn reset(&mut self) {
        self.source_addr = 0;
        self.dest_addr = 0;
        self.word_count = 0;
        self.control = DmaControl::default();
        self.internal_source = 0;
        self.internal_dest = 0;
        self.internal_count = 0;
        self.active = false;
    }

    /// Write source address
    pub fn write_source(&mut self, value: u32) {
        // Mask valid bits based on channel
        let mask = match self.channel_id {
            0 => 0x07FFFFFF, // DMA0: Internal memory only
            1 | 2 => 0x0FFFFFFF, // DMA1-2: Any memory
            3 => 0x0FFFFFFF, // DMA3: Any memory
            _ => 0x0FFFFFFF,
        };
        self.source_addr = value & mask;
    }

    /// Write destination address
    pub fn write_dest(&mut self, value: u32) {
        // Mask valid bits based on channel
        let mask = match self.channel_id {
            0..=2 => 0x07FFFFFF, // DMA0-2: Internal memory only
            3 => 0x0FFFFFFF, // DMA3: Any memory
            _ => 0x0FFFFFFF,
        };
        self.dest_addr = value & mask;
    }

    /// Write word count
    pub fn write_count(&mut self, value: u16) {
        // Maximum count based on channel
        let max_count: u16 = match self.channel_id {
            0 => 0x4000,  // DMA0: 16384 words max
            1 | 2 => 0x4000, // DMA1-2: 16384 words max
            3 => 0,       // DMA3: 65536 words (0 represents 65536)
            _ => 0,
        };
        self.word_count = if value == 0 { max_count } else { value };
    }

    /// Write control register
    pub fn write_control(&mut self, value: u16) {
        let old_enabled = self.control.enabled;
        self.control = DmaControl::from_u16(value);

        // If just enabled, initialize internal registers
        if !old_enabled && self.control.enabled {
            self.reload();
        }

        // Disable if not enabled
        if !self.control.enabled {
            self.active = false;
        }
    }

    /// Read control register
    pub fn read_control(&self) -> u16 {
        self.control.to_u16()
    }

    /// Reload internal registers
    fn reload(&mut self) {
        self.internal_source = self.source_addr;
        self.internal_dest = self.dest_addr;
        self.internal_count = self.word_count;
        
        // Check if should start immediately
        if DmaTiming::from_u8(self.control.timing) == DmaTiming::Immediate {
            self.active = true;
        }
    }

    /// Trigger DMA transfer (for VBlank/HBlank/Special timing)
    pub fn trigger(&mut self, timing: DmaTiming) {
        if !self.control.enabled {
            return;
        }

        if DmaTiming::from_u8(self.control.timing) == timing {
            // Reload if not in repeat mode or first trigger
            if !self.active || !self.control.repeat {
                self.reload();
            }
            self.active = true;
        }
    }

    /// Perform one transfer unit, returns true if transfer complete
    pub fn step_transfer(&mut self) -> bool {
        if !self.active || self.internal_count == 0 {
            return false;
        }

        self.internal_count -= 1;

        // Update addresses based on control
        let transfer_size = self.control.transfer_size();
        
        // Update source address
        match self.control.source_control {
            0 => self.internal_source = self.internal_source.wrapping_add(transfer_size), // Increment
            1 => self.internal_source = self.internal_source.wrapping_sub(transfer_size), // Decrement
            2 => {}, // Fixed
            3 => {}, // Prohibited (increment+reload, not used here)
            _ => {},
        }

        // Update destination address
        match self.control.dest_control {
            0 => self.internal_dest = self.internal_dest.wrapping_add(transfer_size), // Increment
            1 => self.internal_dest = self.internal_dest.wrapping_sub(transfer_size), // Decrement
            2 => {}, // Fixed
            3 => self.internal_dest = self.internal_dest.wrapping_add(transfer_size), // Increment+reload
            _ => {},
        }

        // Check if transfer complete
        if self.internal_count == 0 {
            // Reload destination if mode 3
            if self.control.dest_control == 3 {
                self.internal_dest = self.dest_addr;
            }

            // Disable if not repeat
            if !self.control.repeat {
                self.control.enabled = false;
                self.active = false;
            } else {
                self.active = false; // Wait for next trigger
            }
            
            true // Transfer complete
        } else {
            false
        }
    }

    /// Get current source address for transfer
    pub fn current_source(&self) -> u32 {
        self.internal_source
    }

    /// Get current destination address for transfer
    pub fn current_dest(&self) -> u32 {
        self.internal_dest
    }

    /// Check if should generate IRQ
    pub fn should_irq(&self) -> bool {
        self.control.irq_enable
    }
}
