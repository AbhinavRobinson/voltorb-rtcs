use anyhow::Result;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{message::Message, user::User};

#[derive(Debug)]
pub struct Channel {
    pub id: usize,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
    id_manager: AtomicUsize,
}

impl Channel {
    pub fn new(id: usize, id_manager: AtomicUsize) -> Self {
        Self {
            id,
            users: vec![],
            messages: vec![],
            id_manager,
        }
    }
    pub fn get_id(&self) -> Result<usize> {
        Ok(self.id_manager.fetch_add(1, Ordering::Relaxed))
    }
    pub fn add_users(&mut self, user: User) -> Result<()> {
        Ok(self.users.push(user))
    }
}
