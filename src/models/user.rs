#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub username: String,
}

impl User {
    pub fn new(id: usize, username: String) -> User {
        User { id, username }
    }
}
