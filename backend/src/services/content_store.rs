use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use regex::Regex;

/// 独立页面（关于、归档等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub name: String,
    pub title: String,
    pub draft: bool,
    pub content: String,
}

/// 文章元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMeta {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub draft: bool,
}

/// 完整文章（元信息 + 内容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(flatten)]
    pub meta: PostMeta,
    pub content: String,
}

/// Frontmatter 解析结果
struct ParsedFile {
    frontmatter: String,
    content: String,
}

fn parse_md_file(text: &str) -> Option<ParsedFile> {
    // 支持 TOML frontmatter：+++ 块
    if text.starts_with("+++") {
        let rest = &text[3..];
        if let Some(end) = rest.find("\n+++") {
            let frontmatter = rest[..end].trim().to_string();
            let content = rest[end + 4..].trim_start_matches('\n').to_string();
            return Some(ParsedFile { frontmatter, content });
        }
    }
    None
}

fn extract_string(fm: &toml::Value, key: &str) -> String {
    fm.get(key).and_then(|v| v.as_str()).unwrap_or("").to_string()
}

fn extract_string_array(fm: &toml::Value, key: &str) -> Vec<String> {
    fm.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

fn extract_bool(fm: &toml::Value, key: &str, default: bool) -> bool {
    fm.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

/// 从文件路径解析文章（slug = 文件名去掉 .md）
fn read_post_from_path(path: &std::path::Path) -> Result<Post> {
    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let text = std::fs::read_to_string(path)
        .with_context(|| format!("读取文件失败: {:?}", path))?;

    let parsed = parse_md_file(&text)
        .with_context(|| format!("解析 frontmatter 失败: {:?}", path))?;

    let fm: toml::Value = toml::from_str(&parsed.frontmatter)
        .with_context(|| format!("解析 TOML frontmatter 失败: {:?}", path))?;

    Ok(Post {
        meta: PostMeta {
            slug,
            title: extract_string(&fm, "title"),
            date: extract_string(&fm, "date"),
            description: extract_string(&fm, "description"),
            categories: extract_string_array(&fm, "categories"),
            tags: extract_string_array(&fm, "tags"),
            draft: extract_bool(&fm, "draft", false),
        },
        content: parsed.content,
    })
}

/// 列出所有文章（只含元信息）
pub fn list_posts(site_dir: &PathBuf) -> Result<Vec<PostMeta>> {
    let posts_dir = site_dir.join("content").join("posts");
    let mut posts = Vec::new();

    if !posts_dir.exists() {
        return Ok(posts);
    }

    for entry in std::fs::read_dir(&posts_dir)
        .with_context(|| format!("读取目录失败: {:?}", posts_dir))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            match read_post_from_path(&path) {
                Ok(post) => posts.push(post.meta),
                Err(e) => tracing::warn!("跳过文章 {:?}: {}", path, e),
            }
        }
    }

    // 按日期倒序排列
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(posts)
}

/// 获取单篇文章
pub fn get_post(site_dir: &PathBuf, slug: &str) -> Result<Post> {
    let path = site_dir.join("content").join("posts").join(format!("{}.md", slug));
    read_post_from_path(&path)
}

/// 将文章写入磁盘（创建或更新）
pub fn save_post(site_dir: &PathBuf, post: &Post) -> Result<()> {
    // 验证 slug 安全性（只允许字母、数字、连字符、下划线）
    let slug_re = Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();
    if !slug_re.is_match(&post.meta.slug) {
        anyhow::bail!("slug 格式无效，只允许字母、数字、连字符和下划线");
    }

    let posts_dir = site_dir.join("content").join("posts");
    std::fs::create_dir_all(&posts_dir)?;

    let path = posts_dir.join(format!("{}.md", post.meta.slug));

    let categories_toml = format!(
        "[{}]",
        post.meta.categories.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ")
    );
    let tags_toml = format!(
        "[{}]",
        post.meta.tags.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ")
    );

    let frontmatter = format!(
        "+++\ntitle = \"{}\"\ndate = {}\ndescription = \"{}\"\ncategories = {}\ntags = {}\ndraft = {}\n+++\n\n",
        post.meta.title.replace('"', "\\\""),
        post.meta.date,
        post.meta.description.replace('"', "\\\""),
        categories_toml,
        tags_toml,
        post.meta.draft,
    );

    std::fs::write(&path, format!("{}{}", frontmatter, post.content))
        .with_context(|| format!("写入文章失败: {:?}", path))?;

    Ok(())
}

/// 删除文章
pub fn delete_post(site_dir: &PathBuf, slug: &str) -> Result<()> {
    let slug_re = Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();
    if !slug_re.is_match(slug) {
        anyhow::bail!("slug 格式无效");
    }

    let path = site_dir.join("content").join("posts").join(format!("{}.md", slug));
    if !path.exists() {
        anyhow::bail!("文章不存在: {}", slug);
    }
    std::fs::remove_file(&path)
        .with_context(|| format!("删除文章失败: {:?}", path))?;
    Ok(())
}

/// 获取独立页面（如 about.md、archives.md）
pub fn get_page(site_dir: &PathBuf, name: &str) -> Result<Page> {
    let name_re = Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();
    if !name_re.is_match(name) {
        anyhow::bail!("页面名称格式无效");
    }

    let path = site_dir.join("content").join(format!("{}.md", name));

    // 页面文件不存在时返回空内容
    if !path.exists() {
        return Ok(Page {
            name: name.to_string(),
            title: name.to_string(),
            draft: false,
            content: String::new(),
        });
    }

    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("读取页面失败: {:?}", path))?;

    if let Some(parsed) = parse_md_file(&text) {
        let fm: toml::Value = toml::from_str(&parsed.frontmatter)
            .with_context(|| format!("解析页面 frontmatter 失败: {:?}", path))?;
        Ok(Page {
            name: name.to_string(),
            title: extract_string(&fm, "title"),
            draft: extract_bool(&fm, "draft", false),
            content: parsed.content,
        })
    } else {
        // 无 frontmatter，当作纯内容
        Ok(Page {
            name: name.to_string(),
            title: name.to_string(),
            draft: false,
            content: text,
        })
    }
}

/// 保存独立页面
pub fn save_page(site_dir: &PathBuf, page: &Page) -> Result<()> {
    let name_re = Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();
    if !name_re.is_match(&page.name) {
        anyhow::bail!("页面名称格式无效");
    }

    let content_dir = site_dir.join("content");
    std::fs::create_dir_all(&content_dir)?;

    let path = content_dir.join(format!("{}.md", page.name));

    let frontmatter = format!(
        "+++\ntitle = \"{}\"\ndraft = {}\n+++\n\n",
        page.title.replace('"', "\\\""),
        page.draft,
    );

    std::fs::write(&path, format!("{}{}", frontmatter, page.content))
        .with_context(|| format!("写入页面失败: {:?}", path))?;

    Ok(())
}
