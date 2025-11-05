use std::time::{Duration, Instant};
pub struct PerformanceMetrics {
    step_times: Vec<Duration>,   // N last measurements of step_life()
    render_times: Vec<Duration>, // N last measurements of render()
    last_log: Instant,           // Time of the last display
    sample_size: usize,          // How many frame to average (ex: 60)
}

impl PerformanceMetrics {
    pub fn new(sample_size: usize) -> Self {
        Self {
            step_times: Vec::with_capacity(sample_size),
            render_times: Vec::with_capacity(sample_size),
            last_log: Instant::now(),
            sample_size,
        }
    }

    pub fn record_step(&mut self, duration: Duration) {
        if self.step_times.len() >= self.sample_size {
            self.step_times.remove(0);
        }
        self.step_times.push(duration);
    }

    pub fn record_render(&mut self, duration: Duration) {
        if self.render_times.len() >= self.sample_size {
            self.render_times.remove(0);
        }
        self.render_times.push(duration);
    }

    pub fn avg_step_time(&self) -> Option<Duration> {
        if self.step_times.is_empty() {
            return None;
        }
        let sum: Duration = self.step_times.iter().sum();
        Some(sum / self.step_times.len() as u32)
    }

    pub fn avg_render_time(&self) -> Option<Duration> {
        if self.render_times.is_empty() {
            return None;
        }
        let sum: Duration = self.render_times.iter().sum();
        Some(sum / self.render_times.len() as u32)
    }

    pub fn should_log(&mut self, interval: Duration) -> bool {
        let now = Instant::now();
        if now - self.last_log >= interval {
            self.last_log = now;
            true
        } else {
            false
        }
    }

    pub fn percentile_95_step(&self) -> Option<Duration> {
        if self.step_times.is_empty() {
            return None;
        }
        let mut sorted = self.step_times.clone();
        sorted.sort();
        let idx = (sorted.len() as f64 * 0.95) as usize;
        Some(sorted[idx])
    }
}
