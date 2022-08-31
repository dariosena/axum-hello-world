use axum::{routing::get, Router};
use axum_hello_world::{
    fallback::{api_fallback, app_fallback},
    users::get_user,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let api = Router::new()
        .route("/", get(get_user))
        .fallback(api_fallback);

    let app = Router::new().nest("/api", api).fallback(app_fallback);

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
