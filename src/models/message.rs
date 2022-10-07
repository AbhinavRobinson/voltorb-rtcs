use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Message {
    pub id: usize,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new(id: usize, body: String) -> Message {
        Message {
            body,
            id,
            timestamp: Utc::now(),
        }
    }
}
