#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
}

impl User {
    pub fn new(username: String) -> User {
        User {
            id: '1'.into(),
            username,
        }
    }
}
