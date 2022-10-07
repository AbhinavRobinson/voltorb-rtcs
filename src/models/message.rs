use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Message {
    pub id: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new(body: String) -> Message {
        Message {
            id: '1'.into(),
            body,
            timestamp: Utc::now(),
        }
    }
}
