use std::fmt;
use ripemd256::{Digest, Ripemd256};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use ripemd256;

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>
}
impl User {
    pub fn new(name: &str, pass: & str) -> Self {
        Self {
            id: Uuid::new_v4(),
            username: name.to_string(),
            password: Ripemd256::digest(pass.as_bytes()).to_vec()
        }
    }
    pub fn is_valid_password(&self, pass: String) -> bool {
        let hash = Ripemd256::digest(pass.as_bytes()).to_vec();
        self.password == hash
    }
}
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("id:{} username:{} password:{:?}", self.id.to_hyphenated(),self.username, self.password))
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