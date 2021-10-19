use crate::models::task::Task;
use crate::{models::task::AddTaskReq};
use crate::service::KickService;
use rocket::http::Cookie;
use rocket::{State, http::{CookieJar, Status}, serde::json::Json};

#[post("/add", data = "<model>")]
pub fn add_user(model: Option<Json<AddTaskReq>>, jar: &CookieJar<'_>, service: &State<KickService>) 
    -> (Status, Option<Json<String>>) 
{
    return match model {
        Some(model) => {

            let cookie = jar.get_private_pending("USER")
                            .unwrap_or_else(|| Cookie::new("none", "####"));

            let user = service.get_user(cookie.value());

            match user {
                Some(user) => {
                    let datetime =  model.deadline;               
                    service.add_task(&user.id, Task::new(&model.name, datetime));
                    (Status::Ok, Some(Json(String::from("Ajouté"))))
                }
                None => {
                    (Status::Unauthorized, Some(Json(String::from("Utilisateur non authentifié"))))
                }
            }            
        }
        None => {
            (Status::BadRequest, Some(Json(String::from("Pas de tâche à ajouter"))))
        }
    };
}