use std::fmt;
use rocket::serde::{Deserialize, json::Json};

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UserSignUp {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub password_confirm: String
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} id:{} hash:{}",self.username, self.id, self.password_hash))
    }
}