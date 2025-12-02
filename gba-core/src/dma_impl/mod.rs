/// DMA - Direct Memory Access Controller
/// Modular implementation
mod channel;
mod constants;
mod types;

pub use constants::*;
pub use types::{DmaControl, DmaTiming};

use channel::DmaChannel;

/// DMA Controller (4 channels)
pub struct DMA {
    channels: [DmaChannel; DMA_CHANNEL_COUNT],
}

impl DMA {
    pub fn new() -> Self {
        Self {
            channels: [
                DmaChannel::new(0),
                DmaChannel::new(1),
                DmaChannel::new(2),
                DmaChannel::new(3),
            ],
        }
    }

    /// Reset all DMA channels
    pub fn reset(&mut self) {
        for channel in &mut self.channels {
            channel.reset();
        }
    }

    /// Trigger DMA channels for specific timing
    pub fn trigger(&mut self, timing: DmaTiming) {
        for channel in &mut self.channels {
            channel.trigger(timing);
        }
    }

    /// Perform DMA transfers, returns IRQ flags
    /// Should be called each frame/scanline
    pub fn step<F>(&mut self, mut transfer_fn: F) -> u8
    where
        F: FnMut(u32, u32, bool), // (source, dest, is_32bit)
    {
        let mut irq_flags = 0u8;

        // Process channels in priority order (0 highest, 3 lowest)
        for channel in &mut self.channels {
            if !channel.active {
                continue;
            }

            // Perform all transfers for this channel
            while channel.active {
                let source = channel.current_source();
                let dest = channel.current_dest();
                let is_32bit = channel.control.transfer_32bit;

                // Execute transfer callback
                transfer_fn(source, dest, is_32bit);

                // Step the channel
                let complete = channel.step_transfer();

                if complete {
                    // Check if should generate IRQ
                    if channel.should_irq() {
                        irq_flags |= 1 << channel.channel_id;
                    }
                    break;
                }
            }
        }

        irq_flags
    }

    /// Read DMA register
    pub fn read_register(&self, addr: u32) -> u32 {
        let channel_id = ((addr - DMA0SAD) / 12) as usize;
        if channel_id >= DMA_CHANNEL_COUNT {
            return 0;
        }

        let offset = addr % 12;
        match offset {
            0 => self.channels[channel_id].source_addr,
            4 => self.channels[channel_id].dest_addr,
            8 => self.channels[channel_id].word_count as u32,
            10 => self.channels[channel_id].read_control() as u32,
            _ => 0,
        }
    }

    /// Write DMA register
    pub fn write_register(&mut self, addr: u32, value: u32, is_halfword: bool) {
        let channel_id = ((addr - DMA0SAD) / 12) as usize;
        if channel_id >= DMA_CHANNEL_COUNT {
            return;
        }

        let offset = addr % 12;
        match offset {
            0 => self.channels[channel_id].write_source(value),
            4 => self.channels[channel_id].write_dest(value),
            8 => {
                if is_halfword {
                    self.channels[channel_id].write_count(value as u16);
                }
            }
            10 => {
                if is_halfword {
                    self.channels[channel_id].write_control(value as u16);
                }
            }
            _ => {}
        }
    }

    /// Check if any DMA channel is active
    pub fn is_active(&self) -> bool {
        self.channels.iter().any(|ch| ch.active)
    }

    /// Get active channel (for debugging)
    pub fn active_channel(&self) -> Option<usize> {
        self.channels.iter().position(|ch| ch.active)
    }
}

impl Default for DMA {
    fn default() -> Self {
        Self::new()
    }
}
