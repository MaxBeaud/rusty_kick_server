use std::sync::Mutex;
use crate::models::User;

//add user
//get ref
//contains user
//remove user
pub struct UserService {
    users: Mutex<Vec<User>>
}
impl UserService {
    pub fn new() -> Self {
        Self{users: Mutex::new(Vec::new())}
    }
    pub fn contains_user(&self, name: &str) -> bool {
        let locked = self.users.lock().unwrap();
        for user in locked.to_vec() {
            if user.username == name.to_string() {
                return true;
            }
        }
        false
    }
    pub fn get_users(&self) -> Vec<User> {
        let locked = self.users.lock().unwrap();
        locked.to_vec().to_owned()
    }
    pub fn add(&mut self, value: User) {
        let mut locked = self.users.lock().unwrap();
        locked.push(value);
    }
}