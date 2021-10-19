use std::sync::Mutex;
use uuid::Uuid;
use crate::models::{task::Task, user::User};

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
    pub fn get_users_val(&self) -> Vec<User> {
        let locked = self.users.lock().unwrap();
        locked.to_vec()
    }
    
    pub fn get_user(&self, name: &str) -> Option<User> {
        if self.contains_user(name) {
            for user in self.users.lock().unwrap().iter() {
                if user.username == name {
                    return Some(user.clone());
                }
            }
            None
        }
        else {
            None
        }
    }
    pub fn add_user(&self, value: User) {
        let mut locked = self.users.lock().unwrap();
        locked.push(value);
    }
    pub fn add_task(&self, id: &Uuid, task: Task) {
        for user in self.users.lock().unwrap().iter_mut() {
            if user.id == *id {
                user.tasks.push(task.clone());                
            }
        }
    }
}