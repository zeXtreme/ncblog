use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    path::PathBuf,
};
/// 构建锁使用 tokio::sync::Mutex，以便在 async 函数中跨 await 持有
pub type BuildLock = Arc<tokio::sync::Mutex<()>>;
/// Session 数据
#[derive(Clone, Debug)]
pub struct SessionData {
    pub created_at: std::time::Instant,
}

/// 应用共享状态
#[derive(Clone)]
pub struct AppState {
    /// 管理员密码（明文，个人博客场景）
    pub admin_password: String,
    /// 活跃 session：session_id -> SessionData
    pub sessions: Arc<Mutex<HashMap<String, SessionData>>>,
    /// Hugo 站点根目录（绝对路径）
    pub site_dir: PathBuf,
    /// admin UI dist 目录
    #[allow(dead_code)]
    pub admin_dist_dir: PathBuf,
    /// Hugo 可执行文件路径（默认 "hugo"）
    pub hugo_bin: String,
    /// 构建锁（防止并发构建）
    pub build_lock: BuildLock,
}

impl AppState {
    pub fn new(admin_password: String, site_dir: PathBuf, admin_dist_dir: PathBuf, hugo_bin: String) -> Self {
        Self {
            admin_password,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            site_dir,
            admin_dist_dir,
            hugo_bin,
            build_lock: Arc::new(tokio::sync::Mutex::new(())),
        }
    }

    pub fn is_valid_session(&self, session_id: &str) -> bool {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            // session 有效期 24 小时
            session.created_at.elapsed().as_secs() < 86400
        } else {
            false
        }
    }

    pub fn create_session(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(id.clone(), SessionData {
            created_at: std::time::Instant::now(),
        });
        id
    }

    pub fn remove_session(&self, session_id: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(session_id);
    }
}
