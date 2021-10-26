use super::task::Task;
use ripemd256;
use ripemd256::{Digest, Ripemd256};
use rocket::serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>,
    pub tasks: Vec<Task>,
    pub tasks_len: i32,
}
impl User {
    pub fn new(name: &str, pass: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            username: name.to_string(),
            password: Ripemd256::digest(pass.as_bytes()).to_vec(),
            tasks: Vec::new(),
            tasks_len: 0,
        }
    }
    pub fn is_valid_password(&self, pass: String) -> bool {
        let hash = Ripemd256::digest(pass.as_bytes()).to_vec();
        self.password == hash
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
        self.tasks_len += 1;
    }
    pub fn task(&self, id: Uuid) -> Option<Task> {
        self.tasks.clone().into_iter().find(|x| x.id == id)
    }
    pub fn task_exists(&self, id: Uuid) -> bool {
        self.tasks.iter().any(|x| x.id == id)
    }
}
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "id:{} username:{} password:{}",
            self.id.to_hyphenated(),
            self.username,
            hex::encode(self.password.clone())
        ))
    }
}

#[derive(Deserialize)]
pub struct UserSignUp {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}
#[derive(Deserialize)]
pub struct UserSignIn {
    pub username: String,
    pub password: String,
}
impl fmt::Debug for UserSignUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "name:{} hash:{:X?}",
            self.username,
            self.password.as_bytes()
        ))
    }
}
