use crate::models::task::AddTaskReq;
use crate::models::task::{Task, TaskDetail};
use crate::service::KickService;
use rocket::http::Cookie;
use rocket::{
    http::{CookieJar, Status},
    serde::json::Json,
    State,
};
use uuid::Uuid;

#[post("/add", data = "<model>")]
pub fn add_user(
    model: Option<Json<AddTaskReq>>,
    jar: &CookieJar<'_>,
    service: &State<KickService>,
) -> (Status, Json<String>) {
    return match model {
        Some(model) => {
            let cookie = jar
                .get_private_pending("USER")
                .unwrap_or_else(|| Cookie::new("none", "####"));

            let user = service.get_user(cookie.value());

            match user {
                Some(user) => {
                    let datetime = model.deadline;
                    service.add_task(&user.id, Task::new(&model.name, datetime));
                    (Status::Ok, Json(String::from("Ajouté")))
                }
                None => (
                    Status::Unauthorized,
                    Json(String::from("Utilisateur non authentifié")),
                ),
            }
        }
        None => (
            Status::BadRequest,
            Json(String::from("Pas de tâche à ajouter")),
        ),
    };
}

#[get("/home")]
pub fn get_all_tasks(
    jar: &CookieJar<'_>,
    service: &State<KickService>,
) -> (Status, Json<Option<Vec<Task>>>) {
    let cookie = jar
        .get_private_pending("USER")
        .unwrap_or_else(|| Cookie::new("none", "####"));
    let user = service.get_user(cookie.value());

    match user {
        Some(user) => (Status::Ok, Json(Some(user.tasks.to_vec()))),
        None => (Status::Unauthorized, Json(None)),
    }
}

#[get("/detail/<id>")]
pub fn get_task(
    id: String,
    jar: &CookieJar<'_>,
    service: &State<KickService>,
) -> (Status, Json<Option<TaskDetail>>) {
    let cookie = jar
        .get_private_pending("USER")
        .unwrap_or_else(|| Cookie::new("none", "####"));
    let user = service.get_user(cookie.value());

    match user {
        Some(user) => {
            if let Ok(task_id) = Uuid::parse_str(&id) {
                if let Some(task) = user.task(task_id) {
                    (Status::Ok, Json(Some(TaskDetail::from(task))))
                } else {
                    (Status::NotFound, Json(None))
                }
            } else {
                (Status::BadRequest, Json(None))
            }
        }
        None => (Status::Unauthorized, Json(None)),
    }
}

#[get("/progress/<id>/<value>")]
pub fn update_progress(
    id: String,
    value: f32,
    jar: &CookieJar<'_>,
    service: &State<KickService>,
) -> Status {
    let cookie = jar
        .get_private_pending("USER")
        .unwrap_or_else(|| Cookie::new("none", "####"));
    let user = service.get_user(cookie.value());

    match user {
        Some(user) => {
            if let Ok(task_id) = Uuid::parse_str(&id) {
                if user.task_exists(task_id) {
                    service.update_progress(user.id, task_id, value);
                    Status::Ok
                } else {
                    Status::NotFound
                }
            } else {
                Status::BadRequest
            }
        }
        None => Status::Unauthorized,
    }
}
#[get("/health")]
pub fn health() -> (Status, Json<String>) {
    (
        Status::ImATeapot,
        Json(String::from("Much WOW, very amaze !")),
    )
}
