#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub channel_id: usize,
    pub username: String,
}

impl User {
    pub fn new(id: usize, channel_id: usize, username: String) -> User {
        User {
            id,
            channel_id,
            username,
        }
    }
}
