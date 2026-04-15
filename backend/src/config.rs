use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml::Value;

/// 站点可编辑配置（对应 hugo.toml 中的关键字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: String,
    pub description: String,
    pub author: String,
    pub bio: String,
    pub github: String,
    pub twitter: String,
    pub email: String,
}

/// 从 hugo.toml 读取站点配置
pub fn read_site_config(site_dir: &PathBuf) -> Result<SiteConfig> {
    let config_path = site_dir.join("hugo.toml");
    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("读取 hugo.toml 失败: {:?}", config_path))?;

    let doc: Value = toml::from_str(&content)
        .context("解析 hugo.toml 失败")?;

    let params = doc.get("params").and_then(|v| v.as_table());
    let social = params
        .and_then(|p| p.get("social"))
        .and_then(|v| v.as_table());

    Ok(SiteConfig {
        title: doc.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        base_url: doc.get("baseURL").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        description: doc.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        author: params.and_then(|p| p.get("author")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
        bio: params.and_then(|p| p.get("bio")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
        github: social.and_then(|s| s.get("github")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
        twitter: social.and_then(|s| s.get("twitter")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
        email: social.and_then(|s| s.get("email")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
    })
}

/// 将站点配置写回 hugo.toml（只更新关键字段，保留其余配置）
pub fn write_site_config(site_dir: &PathBuf, cfg: &SiteConfig) -> Result<()> {
    let config_path = site_dir.join("hugo.toml");
    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("读取 hugo.toml 失败: {:?}", config_path))?;

    let mut doc: Value = toml::from_str(&content).context("解析 hugo.toml 失败")?;

    // 更新顶级字段
    if let Value::Table(ref mut table) = doc {
        table.insert("title".to_string(), Value::String(cfg.title.clone()));
        table.insert("baseURL".to_string(), Value::String(cfg.base_url.clone()));
        table.insert("description".to_string(), Value::String(cfg.description.clone()));

        // 确保 params 存在
        let params = table.entry("params".to_string())
            .or_insert(Value::Table(toml::map::Map::new()));

        if let Value::Table(ref mut params_table) = params {
            params_table.insert("author".to_string(), Value::String(cfg.author.clone()));
            params_table.insert("bio".to_string(), Value::String(cfg.bio.clone()));

            // 确保 social 存在
            let social = params_table.entry("social".to_string())
                .or_insert(Value::Table(toml::map::Map::new()));

            if let Value::Table(ref mut social_table) = social {
                social_table.insert("github".to_string(), Value::String(cfg.github.clone()));
                social_table.insert("twitter".to_string(), Value::String(cfg.twitter.clone()));
                social_table.insert("email".to_string(), Value::String(cfg.email.clone()));
            }
        }
    }

    let new_content = toml::to_string_pretty(&doc).context("序列化 hugo.toml 失败")?;
    std::fs::write(&config_path, new_content)
        .with_context(|| format!("写入 hugo.toml 失败: {:?}", config_path))?;

    Ok(())
}
