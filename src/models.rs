use std::fmt;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String
}
impl User {
    pub fn new(name: &str, pass: & str) -> Self {
        User {
            id: Uuid::new_v4(),
            username: name.to_string(),
            password: pass.to_string()
        }
    }
}
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("id:{} username:{} password:{}", self.id.to_hyphenated(),self.username, self.password))
    }
}

#[derive(Deserialize)]
pub struct UserSignUp {
    pub username: String,
    pub password: String,
    pub password_confirm: String
}
#[derive(Deserialize)]
pub struct UserSignIn {
    pub username: String,
    pub password: String,
}

impl fmt::Debug for UserSignUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("name:{} hash:{}",self.username, self.password))
    }
}