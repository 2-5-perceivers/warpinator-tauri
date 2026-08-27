use std::sync::Arc;

/// Callbacks to get a power lock on mobile devices
pub trait PowerManager: Send + Sync + std::fmt::Debug {
    fn acquire_wake_lock(&self);
    fn release_wake_lock(&self);
}

pub struct WakeLockGuard {
    power_manager: Arc<dyn PowerManager>,
}

impl WakeLockGuard {
    pub fn new(power_manager: Arc<dyn PowerManager>) -> Self {
        power_manager.acquire_wake_lock();
        Self { power_manager }
    }
}

impl Drop for WakeLockGuard {
    fn drop(&mut self) {
        self.power_manager.release_wake_lock();
    }
}
