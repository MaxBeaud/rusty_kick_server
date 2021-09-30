use std::fmt;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}
impl User {
    pub fn new(num: i32, name: &str, pass: & str) -> Self {
        User {
            id: num,
            username: name.to_string(),
            password: pass.to_string()
        }
    }
}
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{} id:{} hash:{}",self.username, self.id, self.password))
    }
}

#[derive(Deserialize)]
pub struct UserSignUp {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub password_confirm: String
}

impl fmt::Debug for UserSignUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} id:{} hash:{}",self.username, self.id, self.password))
    }
}