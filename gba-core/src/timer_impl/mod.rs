/// Timer - Hardware Timing System
/// Modular implementation
mod constants;
mod counter;
mod registers;

pub use constants::*;
pub use registers::TimerControl;

use counter::TimerCounter;

/// Timer system (4 hardware timers)
pub struct Timer {
    timers: [TimerCounter; TIMER_COUNT],
}

impl Timer {
    pub fn new() -> Self {
        Self {
            timers: [
                TimerCounter::new(),
                TimerCounter::new(),
                TimerCounter::new(),
                TimerCounter::new(),
            ],
        }
    }

    /// Reset all timers
    pub fn reset(&mut self) {
        for timer in &mut self.timers {
            timer.reset();
        }
    }

    /// Step timers by CPU cycles
    pub fn step(&mut self, cycles: u32) -> u8 {
        let mut irq_flags = 0u8;

        // Process each timer
        for i in 0..TIMER_COUNT {
            let overflow = if i > 0 && self.timers[i].control.count_up {
                // Cascade mode: increment only on previous timer overflow
                false // Will be handled by cascade logic below
            } else {
                // Normal mode: increment by CPU cycles
                self.timers[i].step(cycles)
            };

            // Check for IRQ
            if overflow && self.timers[i].control.irq_enable {
                irq_flags |= 1 << (3 + i); // Timer IRQs are bits 3-6
            }

            // Handle cascade to next timer
            if overflow && i < TIMER_COUNT - 1 {
                let cascade_overflow = self.timers[i + 1].cascade_increment();
                if cascade_overflow && self.timers[i + 1].control.irq_enable {
                    irq_flags |= 1 << (3 + i + 1);
                }
            }
        }

        irq_flags
    }

    /// Read timer register
    pub fn read_register(&self, addr: u32) -> u16 {
        match addr {
            TM0CNT_L => self.timers[0].read_counter(),
            TM0CNT_H => self.timers[0].read_control(),
            TM1CNT_L => self.timers[1].read_counter(),
            TM1CNT_H => self.timers[1].read_control(),
            TM2CNT_L => self.timers[2].read_counter(),
            TM2CNT_H => self.timers[2].read_control(),
            TM3CNT_L => self.timers[3].read_counter(),
            TM3CNT_H => self.timers[3].read_control(),
            _ => 0,
        }
    }

    /// Write timer register
    pub fn write_register(&mut self, addr: u32, value: u16) {
        match addr {
            TM0CNT_L => self.timers[0].write_reload(value),
            TM0CNT_H => self.timers[0].write_control(value),
            TM1CNT_L => self.timers[1].write_reload(value),
            TM1CNT_H => self.timers[1].write_control(value),
            TM2CNT_L => self.timers[2].write_reload(value),
            TM2CNT_H => self.timers[2].write_control(value),
            TM3CNT_L => self.timers[3].write_reload(value),
            TM3CNT_H => self.timers[3].write_control(value),
            _ => {}
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
