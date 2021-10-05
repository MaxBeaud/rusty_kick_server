use chrono::{DateTime, Utc};
use rocket::{ serde::{Deserialize, Serialize}};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub deadline: DateTime<Utc>,
    pub percent_done: i32,
    pub created_on: DateTime<Utc>
}
impl Task {
    pub fn new(name: &str, deadline: DateTime<Utc>, percent_done: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            deadline,
            percent_done,
            created_on: Utc::now()
        } 
    }
    pub fn empty() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from(""),
            deadline: Utc::now(),
            percent_done: 0,
            created_on: Utc::now()
        } 
    }
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskDetail {
    pub id: Uuid,
    pub name: String,
    pub deadline: DateTime<Utc>,
    pub percent_done: i32,
    pub percent_time_spent: i64
}
impl TaskDetail {
    pub fn from(task: Task) -> Self {
        let now = Utc::now();
        let elapsed = task.deadline.signed_duration_since(now);
        let total = task.created_on.signed_duration_since(now);

        Self {
            id: task.id,
            name: task.name,
            deadline: task.deadline,
            percent_done: task.percent_done,
            percent_time_spent: elapsed.num_days() / total.num_days() * 100
        }
    }
}