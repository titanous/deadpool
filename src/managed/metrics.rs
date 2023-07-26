use std::time::{Duration, Instant};

/// Statistics regarding an object returned by the pool
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Metrics {
    /// The instant when this object was created
    pub created: Instant,
    /// The instant when this object was last used
    pub recycled: Option<Instant>,
    /// The number of times the objects was recycled
    pub recycle_count: usize,
    /// The instant when this object was requested
    pub requested: Instant,
    /// The instant when this object was acquired
    pub acquired: Instant,
}

impl Metrics {
    /// Access the age of this object
    pub fn age(&self) -> Duration {
        self.created.elapsed()
    }
    /// Get the time elapsed when this object was last used
    pub fn last_used(&self) -> Duration {
        self.recycled.unwrap_or(self.created).elapsed()
    }
    /// Get the time elapsed to acquire this object
    pub fn acquisition_latency(&self) -> Duration {
        self.acquired.duration_since(self.requested)
    }
    /// Get the time elapsed to create this object if it was not recycled
    pub fn create_latency(&self) -> Option<Duration> {
        if self.recycle_count > 0 {
            return None
        }
        Some(self.created.duration_since(self.requested))
    }
}

impl Default for Metrics {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            created: now,
            recycled: None,
            recycle_count: 0,
            requested: now,
            acquired: now,
        }
    }
}
