use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
    message: String,
}

/// POST /api/login
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    if body.password == state.admin_password {
        let session_id = state.create_session();
        let cookie = format!(
            "session_id={}; HttpOnly; Path=/; SameSite=Strict; Max-Age=86400",
            session_id
        );
        (
            StatusCode::OK,
            [(header::SET_COOKIE, cookie)],
            Json(LoginResponse {
                success: true,
                message: "登录成功".to_string(),
            }),
        )
            .into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                success: false,
                message: "密码错误".to_string(),
            }),
        )
            .into_response()
    }
}

/// POST /api/logout
pub async fn logout(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    if let Some(session_id) = get_session_id(&headers) {
        state.remove_session(&session_id);
    }
    let clear_cookie = "session_id=; HttpOnly; Path=/; Max-Age=0";
    (
        StatusCode::OK,
        [(header::SET_COOKIE, clear_cookie)],
        Json(serde_json::json!({ "success": true })),
    )
        .into_response()
}

/// GET /api/me — 检查登录状态
pub async fn me(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    if is_authenticated(&state, &headers) {
        Json(serde_json::json!({ "authenticated": true })).into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "authenticated": false })),
        )
            .into_response()
    }
}

/// 从 Cookie 中提取 session_id
pub fn get_session_id(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in cookie_header.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("session_id=") {
            return Some(val.to_string());
        }
    }
    None
}

/// 检查请求是否已认证
pub fn is_authenticated(state: &AppState, headers: &axum::http::HeaderMap) -> bool {
    get_session_id(headers)
        .map(|id| state.is_valid_session(&id))
        .unwrap_or(false)
}
