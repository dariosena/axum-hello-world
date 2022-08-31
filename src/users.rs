use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UsersDb = Arc<RwLock<HashMap<Uuid, User>>>;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    id: Uuid,
    name: String,
    username: String,
}

#[derive(Debug, Serialize)]
pub struct GetUserDto {
    name: String,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    name: String,
    username: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn get_users(
    pagination: Option<Query<Pagination>>,
    State(users_db): State<UsersDb>,
) -> impl IntoResponse {
    let users = users_db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let users = users
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(users)
}

pub async fn get_user(
    Path(id): Path<Uuid>,
    State(users_db): State<UsersDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let users = users_db.read().unwrap();

    if let Some(user) = users.get(&id).cloned() {
        let user_dto = GetUserDto {
            name: user.name,
            username: user.username,
        };

        Ok((StatusCode::OK, Json(user_dto)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn create_user(
    State(users_db): State<UsersDb>,
    Json(dto): Json<CreateUserDto>,
) -> impl IntoResponse {
    let user = User {
        id: Uuid::new_v4(),
        name: dto.name,
        username: dto.username,
    };

    users_db.write().unwrap().insert(user.id, user.clone());

    (StatusCode::CREATED, Json(user))
}
