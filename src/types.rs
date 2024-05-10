use serde::{Deserialize, Serialize};

type UUID = String;

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: UUID,
    username: String,
}

impl From<CreateUser> for User {
    fn from(create_user: CreateUser) -> Self {
        User::new(create_user.username)
    }
}

impl User {
    pub fn new(username: String) -> Self {
        User {
            id: uuid::Uuid::new_v4().to_string(),
            username,
        }
    }
}
