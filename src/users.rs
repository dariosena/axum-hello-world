use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetUserDto {
    name: String,
    username: String,
}

pub async fn get_user() -> Json<GetUserDto> {
    let response = GetUserDto {
        name: "Back-end Souls".to_string(),
        username: "backendsouls".to_string(),
    };

    Json(response)
}
