use super::registers::TimerControl;

/// Single hardware timer
#[derive(Debug, Clone)]
pub struct TimerCounter {
    pub counter: u16, // Current counter value
    pub reload: u16,  // Reload value (written to TMxCNT_L)
    pub control: TimerControl,
    pub cycles: u32, // Accumulated cycles for prescaler
}

impl TimerCounter {
    pub fn new() -> Self {
        Self {
            counter: 0,
            reload: 0,
            control: TimerControl::default(),
            cycles: 0,
        }
    }

    /// Reset timer to initial state
    pub fn reset(&mut self) {
        self.counter = 0;
        self.reload = 0;
        self.control = TimerControl::default();
        self.cycles = 0;
    }

    /// Step timer by CPU cycles, returns true if overflow occurred
    pub fn step(&mut self, cpu_cycles: u32) -> bool {
        if !self.control.enabled || self.control.count_up {
            return false;
        }

        self.cycles += cpu_cycles;
        let prescaler = self.control.get_prescaler_cycles();

        let mut overflowed = false;

        while self.cycles >= prescaler {
            self.cycles -= prescaler;

            let (new_counter, overflow) = self.counter.overflowing_add(1);
            self.counter = new_counter;

            if overflow {
                // Reload on overflow
                self.counter = self.reload;
                overflowed = true;
            }
        }

        overflowed
    }

    /// Cascade increment (from previous timer overflow)
    pub fn cascade_increment(&mut self) -> bool {
        if !self.control.enabled || !self.control.count_up {
            return false;
        }

        let (new_counter, overflow) = self.counter.overflowing_add(1);
        self.counter = new_counter;

        if overflow {
            self.counter = self.reload;
            true
        } else {
            false
        }
    }

    /// Read counter value
    pub fn read_counter(&self) -> u16 {
        self.counter
    }

    /// Write reload value (also resets counter if timer is disabled)
    pub fn write_reload(&mut self, value: u16) {
        self.reload = value;
        if !self.control.enabled {
            self.counter = value;
        }
    }

    /// Read control register
    pub fn read_control(&self) -> u16 {
        self.control.to_u16()
    }

    /// Write control register
    pub fn write_control(&mut self, value: u16) {
        let old_enabled = self.control.enabled;
        self.control = TimerControl::from_u16(value);

        // If timer just got enabled, reload counter
        if !old_enabled && self.control.enabled {
            self.counter = self.reload;
            self.cycles = 0;
        }
    }
}

impl Default for TimerCounter {
    fn default() -> Self {
        Self::new()
    }
}
