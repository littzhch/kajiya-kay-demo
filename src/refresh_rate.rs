use std::time::{Duration, Instant};

pub struct RefreshRate {
    interval: Duration,
    last_update: Instant,
}

impl RefreshRate {
    pub fn new(rate: f64) -> Self {
        Self {
            interval: Duration::from_secs_f64(1.0 / rate),
            last_update: Instant::now(),
        }
    }

    /// tell RefreshRate that you will refresh now and get the next refresh time 
    pub fn refresh_now(&mut self) -> Instant {
        let next_update = self.last_update + self.interval;
        self.last_update = Instant::now();
        next_update
    }
    
    /// get interval between refreshes
    pub fn interval(&self) -> Duration {
        self.interval
    }
}
