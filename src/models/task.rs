use chrono::{NaiveDate, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub deadline: NaiveDate,
    pub percent_done: f32,
    pub created_on: NaiveDate,
}
impl Task {
    pub fn new(name: &str, deadline: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            deadline,
            percent_done: 0.0,
            created_on: Utc::now().naive_local().date(),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskDetail {
    pub id: Uuid,
    pub name: String,
    pub deadline: NaiveDate,
    pub percent_done: f32,
    pub percent_time_spent: i64,
}
impl TaskDetail {
    pub fn from(task: Task) -> Self {
        let now = Utc::now().naive_local().date();
        let elapsed = task.deadline.signed_duration_since(now);
        let total = task.created_on.signed_duration_since(now);

        Self {
            id: task.id,
            name: task.name,
            deadline: task.deadline,
            percent_done: task.percent_done,
            percent_time_spent: if !total.num_days() == 0 {
                elapsed.num_days() / total.num_days() * 100
            } else {
                0
            },
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct AddTaskReq {
    pub name: String,
    pub deadline: NaiveDate,
}
