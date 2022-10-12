pub mod models;

use crate::models::{channel::Channel, message::Message, user::User};
use anyhow::{Ok, Result};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() -> Result<()> {
    let global_id = AtomicUsize::new(1);
    let mut channel = Channel::new(
        global_id.fetch_add(1, Ordering::Relaxed),
        AtomicUsize::new(1),
    );
    let message = Message::new(
        channel.get_id().expect("Error creating ID."),
        channel.id,
        "Hello World!".into(),
    );
    let user = User::new(
        channel.get_id().expect("Error creating ID."),
        channel.id,
        "Abhinav Robinson".into(),
    );
    let chat_result = channel.add_users(user).is_ok();
    let message_result = channel.add_messages(message).is_ok();
    if chat_result && message_result {
        println!(
            "{} says {}",
            channel.users[0].username, channel.messages[0].body
        );
        println!("{:?}", channel);
    }
    Ok(())
}
