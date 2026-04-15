use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    routes::auth::is_authenticated,
    services::content_store::{self, Page},
};

fn require_auth(state: &AppState, headers: &axum::http::HeaderMap) -> Option<Response> {
    if !is_authenticated(state, headers) {
        Some((StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "未授权" }))).into_response())
    } else {
        None
    }
}

/// GET /api/pages/:name
pub async fn get_page(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(name): Path<String>,
) -> Response {
    if let Some(err) = require_auth(&state, &headers) { return err; }

    match content_store::get_page(&state.site_dir, &name) {
        Ok(page) => Json(page).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

/// PUT /api/pages/:name
pub async fn update_page(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(name): Path<String>,
    Json(mut page): Json<Page>,
) -> Response {
    if let Some(err) = require_auth(&state, &headers) { return err; }

    // name 由路径参数决定
    page.name = name;

    match content_store::save_page(&state.site_dir, &page) {
        Ok(_) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}
