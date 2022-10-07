pub mod models;

use anyhow::{Ok, Result};

use crate::models::message::Message;

fn main() -> Result<()> {
    let message: Message = Message::new("Hello World".into());
    println!("{:?}", message);
    Ok(())
}
