use std::time::{Duration, Instant};

pub struct FpsWindow {
    window_size: u64,
    count: u64,
    started_at: Instant,
}

impl FpsWindow {
    pub fn new(window_size: u64) -> Self {
        Self {
            window_size: window_size.max(1),
            count: 0,
            started_at: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> Option<(f64, u64, Duration)> {
        self.count += 1;
        if self.count < self.window_size {
            return None;
        }

        let elapsed = self.started_at.elapsed();
        let fps = self.count as f64 / elapsed.as_secs_f64().max(1e-9);
        let frames = self.count;

        self.count = 0;
        self.started_at = Instant::now();

        Some((fps, frames, elapsed))
    }
}
