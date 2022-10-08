pub mod models;

use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Ok, Result};

use crate::models::{channel::Channel, message::Message, user::User};

fn main() -> Result<()> {
    let global_id = AtomicUsize::new(1);
    let channel = Channel::new(
        global_id.fetch_add(1, Ordering::Relaxed),
        AtomicUsize::new(1),
    );
    let message = Message::new(channel.get_id(), channel.id, "Hello World!".into());
    let user = User::new(channel.get_id(), channel.id, "Abhinav Robinson".into());
    println!("{} says {}", user.username, message.body);
    println!("{:?}", message);
    Ok(())
}
