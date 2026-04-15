use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    config::{read_site_config, write_site_config, SiteConfig},
    routes::auth::is_authenticated,
};

/// GET /api/site-config
pub async fn get_site_config(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Response {
    if !is_authenticated(&state, &headers) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "未授权" }))).into_response();
    }

    match read_site_config(&state.site_dir) {
        Ok(cfg) => Json(cfg).into_response(),
        Err(e) => {
            tracing::error!("读取站点配置失败: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

/// PUT /api/site-config
pub async fn update_site_config(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(cfg): Json<SiteConfig>,
) -> Response {
    if !is_authenticated(&state, &headers) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "未授权" }))).into_response();
    }

    match write_site_config(&state.site_dir, &cfg) {
        Ok(_) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            tracing::error!("写入站点配置失败: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}
