use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    routes::auth::is_authenticated,
    services::hugo_runner::run_hugo_build,
};

/// POST /api/build — 触发 Hugo 构建
pub async fn trigger_build(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Response {
    if !is_authenticated(&state, &headers) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "未授权" }))).into_response();
    }

    // 尝试获取构建锁（非阻塞）
    let _lock = match state.build_lock.try_lock() {
        Ok(lock) => lock,
        Err(_) => {
            return (
                StatusCode::CONFLICT,
                Json(serde_json::json!({ "error": "构建正在进行中，请稍后再试" })),
            ).into_response();
        }
    };

    tracing::info!("开始 Hugo 构建...");

    match run_hugo_build(&state.site_dir, &state.hugo_bin).await {
        Ok(result) => {
            if result.success {
                tracing::info!("Hugo 构建成功");
                Json(serde_json::json!({
                    "success": true,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                })).into_response()
            } else {
                tracing::error!("Hugo 构建失败: {}", result.stderr);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "success": false,
                        "stdout": result.stdout,
                        "stderr": result.stderr,
                    })),
                ).into_response()
            }
        }
        Err(e) => {
            tracing::error!("运行 Hugo 失败: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            ).into_response()
        }
    }
}
