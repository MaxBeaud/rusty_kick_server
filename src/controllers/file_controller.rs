use rocket::{Data, State, http::{Cookie, CookieJar, Status}, serde::json::Json, tokio::{fs::OpenOptions, io::AsyncWriteExt}};
use uuid::Uuid;
use crate::{models::{task::TaskDetail, user::User}, service::KickService};

#[post("/images", data = "<image>")]
pub async fn upload_image(jar: &CookieJar<'_>, service: &State<KickService>, image: Data<'_>) -> (Status, Json<String>) {
    let cookie = jar
        .get_private_pending("USER")
        .unwrap_or_else(|| Cookie::new("none", "####"));
    let user = service.get_user(cookie.value());

    match user {
        Some(user) => {
            let image_id = Uuid::new_v4().to_string();
            let file_path = format!("src/static/images/{}.jpg", image_id);
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(file_path)
                .await.unwrap();

            file.write_all(image.).unwrap();

            (Status::Ok, Json(image_id))
        },
        None => ( Status::Unauthorized, Json("None".to_string()) ),
    }
}