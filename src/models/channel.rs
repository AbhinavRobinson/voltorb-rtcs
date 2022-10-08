use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Channel {
    pub id: usize,
    id_manager: AtomicUsize,
}

impl Channel {
    pub fn new(id: usize, id_manager: AtomicUsize) -> Self {
        Self { id, id_manager }
    }
    pub fn get_id(&self) -> usize {
        self.id_manager.fetch_add(1, Ordering::Relaxed)
    }
}
