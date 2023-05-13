use std::time::{self, Instant};

pub struct Time {
    pub time_to_pass: f32,
    pub start: Instant,
}

impl Default for Time {
    fn default() -> Self {
        let start = time::Instant::now();
        Time {
            time_to_pass: 0.0,
            start,
        }
    }
}

impl Time {
    pub fn tick(&mut self) -> bool {
        if self.start.elapsed().as_secs_f32() > self.time_to_pass {
            self.start = time::Instant::now();
            return true;
        }
        false
    }
}
