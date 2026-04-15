use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    routes::auth::is_authenticated,
    services::content_store::{self, Post},
};

fn require_auth(state: &AppState, headers: &axum::http::HeaderMap) -> Option<Response> {
    if !is_authenticated(state, headers) {
        Some((StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "未授权" }))).into_response())
    } else {
        None
    }
}

/// GET /api/posts
pub async fn list_posts(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Response {
    if let Some(err) = require_auth(&state, &headers) { return err; }

    match content_store::list_posts(&state.site_dir) {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => {
            tracing::error!("列出文章失败: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

/// GET /api/posts/:slug
pub async fn get_post(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    if let Some(err) = require_auth(&state, &headers) { return err; }

    match content_store::get_post(&state.site_dir, &slug) {
        Ok(post) => Json(post).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

/// POST /api/posts
pub async fn create_post(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(post): Json<Post>,
) -> Response {
    if let Some(err) = require_auth(&state, &headers) { return err; }

    match content_store::save_post(&state.site_dir, &post) {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({ "success": true, "slug": post.meta.slug }))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

/// PUT /api/posts/:slug
pub async fn update_post(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(slug): Path<String>,
    Json(mut post): Json<Post>,
) -> Response {
    if let Some(err) = require_auth(&state, &headers) { return err; }

    // slug 由路径参数决定，防止客户端篡改
    post.meta.slug = slug;

    match content_store::save_post(&state.site_dir, &post) {
        Ok(_) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

/// DELETE /api/posts/:slug
pub async fn delete_post(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    if let Some(err) = require_auth(&state, &headers) { return err; }

    match content_store::delete_post(&state.site_dir, &slug) {
        Ok(_) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}
