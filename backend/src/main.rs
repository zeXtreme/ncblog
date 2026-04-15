mod app_state;
mod config;
mod routes;
mod services;

use std::net::SocketAddr;
use std::path::PathBuf;

use axum::{
    http::{header, Method},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing_subscriber::EnvFilter;

use app_state::AppState;
use routes::{auth, build, pages, posts, site};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("ncblog=info".parse().unwrap()))
        .init();

    // 确定项目根目录
    let project_root = find_project_root().expect("无法找到项目根目录（缺少 site/hugo.toml）");
    tracing::info!("项目根目录: {}", project_root.display());

    let site_dir = project_root.join("site");
    let admin_dist_dir = project_root.join("admin-ui").join("dist");
    let hugo_bin = std::env::var("HUGO_BIN").unwrap_or_else(|_| "hugo".to_string());
    let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    tracing::info!("Hugo 二进制: {}", hugo_bin);
    tracing::info!("管理员界面目录: {}", admin_dist_dir.display());

    let state = AppState::new(admin_password, site_dir, admin_dist_dir.clone(), hugo_bin);

    // CORS 配置（仅开发时需要，生产由同源服务）
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::COOKIE, header::SET_COOKIE])
        .allow_origin(Any);

    // API 路由
    let api_router = Router::new()
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
        .route("/posts", get(posts::list_posts).post(posts::create_post))
        .route(
            "/posts/:slug",
            get(posts::get_post)
                .put(posts::update_post)
                .delete(posts::delete_post),
        )
        .route("/site-config", get(site::get_site_config).put(site::update_site_config))
        .route("/pages/:name", get(pages::get_page).put(pages::update_page))
        .route("/build", post(build::trigger_build));

    // Hugo 生成的静态站点（博客首页）
    let public_service = ServeDir::new(state.site_dir.join("public"));

    // 构建主路由
    let app = Router::new()
        .nest("/api", api_router)
        // Hugo 生成的公开站点挂载到根路径 /
        .nest_service("/", public_service)
        // 管理界面占位页（未构建时）
        .fallback(admin_fallback)
        .layer(cors)
        .with_state(state.clone());

    // 如果 admin-ui/dist 目录存在，挂载到 /admin
    let app = if admin_dist_dir.exists() {
        tracing::info!("挂载管理界面静态文件: {}", admin_dist_dir.display());
        let admin_service = ServeDir::new(&admin_dist_dir)
            .fallback(axum::routing::get_service(tower_http::services::ServeFile::new(
                admin_dist_dir.join("index.html"),
            )));
        app.nest_service("/admin", admin_service)
    } else {
        tracing::warn!("管理界面未构建，访问 /admin 将显示占位页面。运行 `cd admin-ui && npm run build` 构建");
        app
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("服务启动在 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// 当 admin-ui/dist 未构建时返回的占位页面
async fn admin_fallback() -> impl IntoResponse {
    Html(r#"<!DOCTYPE html>
<html lang="zh">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ncblog 管理后台</title>
  <style>
    body { font-family: system-ui, sans-serif; display: flex; align-items: center; justify-content: center;
           min-height: 100vh; margin: 0; background: #f5f5f5; }
    .card { background: white; padding: 2rem 3rem; border-radius: 12px; box-shadow: 0 2px 16px rgba(0,0,0,.08);
            max-width: 480px; text-align: center; }
    h1 { margin-top: 0; color: #2f6fed; }
    code { background: #f0f0f0; padding: .2em .5em; border-radius: 4px; font-size: .9em; }
    .step { text-align: left; margin: 1rem 0; }
    .step p { margin: .25rem 0; }
  </style>
</head>
<body>
  <div class="card">
    <h1>ncblog 管理后台</h1>
    <p>管理界面尚未构建。请运行以下命令：</p>
    <div class="step">
      <p><code>cd admin-ui</code></p>
      <p><code>npm install</code></p>
      <p><code>npm run build</code></p>
    </div>
    <p>构建完成后访问 <a href="/admin">/admin</a> 进入管理界面。</p>
    <hr>
    <p style="color:#666;font-size:.9em">API 服务正常运行中 &middot; <a href="/">查看博客</a></p>
  </div>
</body>
</html>"#)
}

/// 从当前目录向上查找包含 site/hugo.toml 的项目根目录
fn find_project_root() -> Option<PathBuf> {
    // 优先使用环境变量
    if let Ok(root) = std::env::var("PROJECT_ROOT") {
        let path = PathBuf::from(root);
        if path.join("site").join("hugo.toml").exists() {
            return Some(path);
        }
    }

    // 从当前工作目录向上遍历
    let mut dir = std::env::current_dir().ok()?;
    for _ in 0..6 {
        if dir.join("site").join("hugo.toml").exists() {
            return Some(dir);
        }
        if !dir.pop() {
            break;
        }
    }

    // 从可执行文件路径向上遍历
    let mut exe_dir = std::env::current_exe().ok()?;
    exe_dir.pop(); // 移除可执行文件名
    for _ in 0..6 {
        if exe_dir.join("site").join("hugo.toml").exists() {
            return Some(exe_dir.clone());
        }
        if !exe_dir.pop() {
            break;
        }
    }

    None
}
