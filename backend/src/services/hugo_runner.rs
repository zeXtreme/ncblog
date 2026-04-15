use anyhow::{Context, Result};
use serde::Serialize;
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Debug, Serialize)]
pub struct BuildResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

/// 运行 Hugo 构建（同步封装，在 tokio 线程池中执行）
pub async fn run_hugo_build(site_dir: &PathBuf, hugo_bin: &str) -> Result<BuildResult> {
    let output = Command::new(hugo_bin)
        .arg("--gc")
        .arg("--minify")
        .current_dir(site_dir)
        .output()
        .await
        .with_context(|| format!("启动 Hugo 失败，请确保 '{}' 已安装并在 PATH 中", hugo_bin))?;

    Ok(BuildResult {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}
