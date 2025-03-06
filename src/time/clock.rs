// per Pan Docs: A “dot” = one 2^22 Hz (≅ 4.194 MHz) time unit.

/// Represents a clock in the Game Boy hardware that ticks at a specific frequency.
pub struct Clock {
    name: String,
    mcycles_per_period: u32,
    mcycles_since_tick: u32,

    pub debug_total_ticks: u64,
}

impl Clock {
    pub fn new(name: impl Into<String>, mcycles_per_period: u32) -> Self {
        Self {
            name: name.into(),
            mcycles_per_period,
            mcycles_since_tick: 0,
            debug_total_ticks: 0,
        }
    }

    pub fn set_period(&mut self, mcycles_per_period: u32) {
        self.mcycles_per_period = mcycles_per_period;
    }

    pub fn update_and_check(&mut self) -> bool {
        self.mcycles_since_tick += 1;
        if self.mcycles_since_tick >= self.mcycles_per_period {
            self.mcycles_since_tick = 0;
            self.debug_total_ticks += 1;
            return true;
        }
        return false;
    }

    pub fn print(&self) {
        println!("Simple clock {}", self.name);
        println!("  period: {} m-cycles", self.mcycles_per_period);
        println!("  count:  {} m-cycles", self.mcycles_since_tick);
        println!("  total ticks: {}", self.debug_total_ticks);
    }
}
