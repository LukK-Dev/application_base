use std::time::{Duration, Instant};

pub struct Timing {
    start: Instant,
    last: Instant,
    delta: Duration,
}

impl Timing {
    pub fn new() -> Timing {
        Self {
            start: Instant::now(),
            last: Instant::now(),
            delta: Duration::ZERO,
        }
    }

    pub fn update(&mut self) {
        self.delta = Instant::now() - self.last;
        self.last = Instant::now()
    }

    pub fn time_delta(&self) -> Duration {
        self.delta
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.delta.as_secs_f32()
    }

    pub fn time_since_startup(&self) -> Duration {
        Instant::now() - self.start
    }
}
