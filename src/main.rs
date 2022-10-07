pub mod models;

use std::sync::atomic::AtomicUsize;

use anyhow::{Ok, Result};

use crate::models::{channel::Channel, message::Message, user::User};

fn main() -> Result<()> {
    let channel = Channel::new(AtomicUsize::new(1));
    let message = Message::new(channel.get_id(), "Hello World!".into());
    let user = User::new(channel.get_id(), "Abhinav Robinson".into());
    println!("{} says {}", user.username, message.body);
    Ok(())
}
