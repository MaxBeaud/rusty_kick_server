use crate::models::task::Task;
use crate::{models::task::AddTaskReq};
use crate::service::KickService;
use rocket::http::Cookie;
use rocket::{State, http::{CookieJar, Status}, serde::json::Json};

#[post("/add", data = "<model>")]
pub async fn add_user(model: Option<Json<AddTaskReq>>, jar: &CookieJar<'_>, service: &State<KickService>) 
    -> (Status, Option<Json<String>>) 
{
    match model {
        Some(model) => {

            let cookie = jar.get_private_pending("USER")
                            .unwrap_or_else(|| Cookie::new("none", "####"));

            let user = service.get_user(cookie.value());
            match user {
                Some(mut user) => {
                    let datetime =  model.deadline;
                    user.tasks.push(Task::new(&model.name, datetime));
                    (Status::Ok, None)
                }
                None => {
                    (Status::Unauthorized, Some(Json(String::from("Utilisateur non authentifié"))))
                }
            }            
        }
        None => {
            (Status::BadRequest, Some(Json(String::from("Pas de tâche à ajouter"))))
        }
    }
}