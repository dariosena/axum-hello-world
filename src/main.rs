use axum::{routing::get, Router};
use axum_hello_world::{
    fallback::{api_fallback, app_fallback},
    users::{create_user, delete_user, get_user, get_users, update_user, UsersDb},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let users_db = UsersDb::default();

    let users_api = Router::with_state(users_db)
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user));

    let api = Router::new()
        .nest("/users", users_api)
        .fallback(api_fallback);
    let app = Router::new().nest("/api", api).fallback(app_fallback);

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
