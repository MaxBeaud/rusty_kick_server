use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, PartialEq, Clone)]
pub struct Task {
    pub name: String,
    pub deadline: DateTime<Utc>
}
impl Task {
    
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskDetail {
    pub id: Uuid,
    pub name: String,
    pub deadline: DateTime<Utc>,
    pub percent_done: i32,
    pub percent_time_spent: i32
}
impl TaskDetail {
    
}