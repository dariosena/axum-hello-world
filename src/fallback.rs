use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404, "message": "Not Found"
    });

    (StatusCode::NOT_FOUND, Json(body))
}

pub async fn app_fallback() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "Not Found".to_string())
}
