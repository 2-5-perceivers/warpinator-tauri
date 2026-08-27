use std::fmt::Debug;

#[uniffi::export(callback_interface)]
pub trait PowerManager: Send + Sync {
    fn acquire_wake_lock(&self);
    fn release_wake_lock(&self);
}

pub(crate) struct PowerManagerWrapper {
    inner: Box<dyn PowerManager>,
}

impl PowerManagerWrapper {
    pub fn new(inner: Box<dyn PowerManager>) -> Self {
        Self { inner }
    }
}

impl Debug for PowerManagerWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PowerManagerWrapper").finish()
    }
}

impl warpinator_lib::power_manager::PowerManager for PowerManagerWrapper {
    fn acquire_wake_lock(&self) {
        self.inner.acquire_wake_lock();
    }

    fn release_wake_lock(&self) {
        self.inner.release_wake_lock();
    }
}
