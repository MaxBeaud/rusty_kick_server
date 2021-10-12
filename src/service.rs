use std::sync::Mutex;
use crate::models::user::User;


pub struct KickService {
    users: Mutex<Vec<User>>
}
impl KickService {
    pub fn new() -> Self {
        Self{users: Mutex::new(Vec::new())}
    }
    pub fn contains_user(&self, name: &str) -> bool {
        let locked = self.users.lock().unwrap();
        for user in locked.to_vec() {
            if user.username == name {
                return true;
            }
        }
        false
    }
    pub fn get_users(&self) -> Vec<User> {
        let locked = self.users.lock().unwrap();
        locked.to_vec()
    }
    pub fn get_user(&self, name: &str) -> Option<User> {
        let locked = self.users.lock().unwrap();
        let vec = locked.to_vec();
        if self.contains_user(name) {
            for user in vec {
                if user.username == name {
                    return Some(user);
                }
            }
            None
        }
        else {
            None
        }   
    }
    pub fn add(&self, value: User) {
        let mut locked = self.users.lock().unwrap();
        locked.push(value);
    }
}