use crate::models::{task::Task, user::User};
use rayon::prelude::*;
use std::sync::Mutex;
use uuid::Uuid;

pub struct KickService {
    users: Mutex<Vec<User>>,
}
impl KickService {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
        }
    }
    pub fn contains_user(&self, name: &str) -> bool {
        let locked = self.users.lock().unwrap();
        let users = locked.to_vec();
        users.par_iter().any(|user| user.username == name)
    }
    pub fn get_users_val(&self) -> Vec<User> {
        let locked = self.users.lock().unwrap();
        locked.to_vec()
    }

    pub fn get_user(&self, name: &str) -> Option<User> {
        if self.contains_user(name) {
            self.users
                .lock()
                .unwrap()
                .par_iter()
                .find_first(|&user| user.username == name)
                .cloned()
        } else {
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
                user.add_task(task.clone());
            }
        }
    }
    pub fn update_progress(&self, user_id: Uuid, task_id: Uuid, value: f32) {
        for user in self.users.lock().unwrap().iter_mut() {
            if user.id == user_id {
                user.tasks
                    .par_iter_mut()
                    .find_first(|t| t.id == task_id)
                    .unwrap()
                    .percent_done = value;
            }
        }
    }
}
