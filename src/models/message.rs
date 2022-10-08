use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Message {
    pub id: usize,
    pub channel_id: usize,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new(id: usize, channel_id: usize, body: String) -> Message {
        Message {
            id,
            channel_id,
            body,
            timestamp: Utc::now(),
        }
    }
}
