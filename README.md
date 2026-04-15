# ncblog

A modern minimalist two-column personal blog. Hugo generates the static site; a Rust/Axum backend provides a REST API; a React + Milkdown admin UI lets you write and publish posts without touching the command line.

```
ncblog/
  backend/        # Rust (Axum) API server
  admin-ui/       # React + Vite + Milkdown admin interface
  site/           # Hugo static site (themes/minimal)
```

## Prerequisites

| Tool | Version |
|------|---------|
| [Hugo](https://gohugo.io/installation/) | ≥ 0.120 (extended) |
| [Rust](https://rustup.rs/) | stable (≥ 1.75) |
| [Node.js](https://nodejs.org/) | ≥ 18 |

## Docker deployment (recommended)

No need to install Rust, Node.js, or Hugo on the host.

```bash
docker compose up --build
```

- Blog: **http://localhost:3000** (after first build, click **触发构建** in the admin panel)
- Admin panel: **http://localhost:3000/admin**

Set a secure password before exposing publicly:

```bash
ADMIN_PASSWORD=yourpassword docker compose up --build
```

Articles (`site/content/`) and site config (`site/hugo.toml`) are bind-mounted from the host directory, so data persists across container rebuilds.

To update Hugo version, change `HUGO_VERSION` in the Dockerfile (default: `0.124.1`).

---

## Manual setup

### 1. Build the admin UI

```bash
cd admin-ui
npm install
npm run build        # outputs to admin-ui/dist/
```

### 2. Start the backend

```bash
cd backend
cargo run --release
```

The server starts on **http://localhost:3000**.

Default password is `admin123`, overridable via the `ADMIN_PASSWORD` environment variable.

### 3. Use the admin panel

Open **http://localhost:3000/admin** → log in → write posts → click **触发构建** → the blog is rebuilt.

The generated site is served at **http://localhost:3000/**.

---

## Development

Run the admin UI dev server with hot-reload (proxies `/api` to the Rust backend):

```bash
# Terminal 1
cd backend && cargo run

# Terminal 2
cd admin-ui && npm run dev   # http://localhost:5173
```

Preview the Hugo theme without the backend:

```bash
cd site && hugo server       # http://localhost:1313
```

## Configuration

All configuration is via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `ADMIN_PASSWORD` | `admin123` | Admin login password |
| `PORT` | `3000` | Listening port |
| `PROJECT_ROOT` | auto-detect | Absolute path to repo root |
| `HUGO_BIN` | `hugo` | Path to Hugo executable |

Site metadata (title, author, social links, etc.) is edited through the admin panel's **站点设置** page, which writes directly to `site/hugo.toml`.

## API

All endpoints require a session cookie (obtained via `POST /api/login`).

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/login` | `{password}` → sets `session_id` cookie |
| POST | `/api/logout` | Clears session |
| GET | `/api/me` | 200 if authenticated, 401 otherwise |
| GET | `/api/posts` | List all posts (frontmatter only) |
| GET | `/api/posts/:slug` | Full post (frontmatter + content) |
| POST | `/api/posts` | Create post |
| PUT | `/api/posts/:slug` | Update post |
| DELETE | `/api/posts/:slug` | Delete post |
| GET | `/api/site-config` | Read `site/hugo.toml` params |
| PUT | `/api/site-config` | Write `site/hugo.toml` params |
| GET | `/api/pages/:name` | Get standalone page (about, archives…) |
| PUT | `/api/pages/:name` | Save standalone page |
| POST | `/api/build` | Run `hugo` and return output |

## Post format

Posts are stored as `site/content/posts/{slug}.md` with TOML frontmatter:

```markdown
+++
title = "Hello World"
date = 2026-04-14T09:00:00Z
description = "A short summary"
categories = ["Tech"]
tags = ["rust", "hugo"]
draft = false
+++

Your content here...
```
