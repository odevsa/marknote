<div align="center">
  <img src="frontend/static/favicon.svg" width="80" height="80" alt="MarkNote Logo" />
  
  **MarkNote**

  **A self-hosted, privacy-first, lightweight Markdown note-taking.**

  ![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg?logo=rust&style=flat-square)
  ![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00.svg?logo=svelte&style=flat-square)
  ![SQLite](https://img.shields.io/badge/SQLite-FTS5-003B57.svg?logo=sqlite&style=flat-square)
  ![Docker](https://img.shields.io/badge/Docker-Ready-2496ED.svg?logo=docker&style=flat-square)


  [Features](#features) •
  [Themes](#themes) •
  [Quick Start](#quick-start-with-docker) •
  [Configuration](#configuration) •
  [Development](#development) •
  [Tech Stack](#tech-stack)
</div>


## Overview

<div align="center">
  <img src="assets/screenshot-1.png" alt="Workspace Overview" width="49%" />
  <img src="assets/screenshot-2.png" alt="Markdown Editor Split View" width="49%" />
  <br />
  <img src="assets/screenshot-3.png" alt="Full-Text Search" width="49%" />
  <img src="assets/screenshot-4.png" alt="Settings & Customization" width="49%" />
</div>

**MarkNote** is designed for users who want total control over their notes without being locked into proprietary database formats or cloud services. 

All your notes are stored directly as plain `.md` files in a regular folder on your machine or server. You can edit them with MarkNote, sync them with Syncthing or Git, or open them in any editor (Obsidian, Neovim, VS Code) seamlessly.

## Features

- **Direct Filesystem Persistence**: No vendor lock-in. Notes remain plain `.md` files in standard directory hierarchies.
- **Lightning Fast FTS5 Search**: Instant full-text search across all notes powered by SQLite FTS5 and automatic file-watcher synchronization (`notify`).
- **CodeMirror 6 Markdown Editor**: Rich syntax highlighting, line numbers, shortcuts (`Ctrl+S`, `Ctrl+K`), formatting toolbar, and live side-by-side preview.
- **Smart Auto-Save & Local Draft Recovery**:
  - Configurable auto-save toggle and delay (0.5s to 5.0s).
  - Browser `localStorage` draft protection when auto-save is off, with prompt to restore or discard drafts on open.
- **Deep Linking & Direct Navigation**: Bookmarking or sharing links directly to specific files and modes (`/file/folder/note.md`, `/file/.../edit`, `/file/.../split`).
- **Dynamic Theme System**: Includes Dark, Light, Paper (sepia book theme), Code (VS Code dark), Matrix, and System auto-detect.
- **Internationalization (i18n)**: Built-in support for 5 languages:
  - English (`en`)
  - Português do Brasil (`pt-BR`)
  - Español (`es`)
  - Français (`fr`)
  - Italiano (`it`)
- **Privacy-First & Secure**: Self-hosted, single user/admin setup with Argon2id password hashing and HTTP-only JWT sessions.
- **Single Binary Distribution**: Backend in Rust (Axum) embeds the frontend SPA for lightweight deployment with minimal RAM footprint.

## Themes

MarkNote features dynamic theme support tailored for different writing environments and lighting preferences:

<div align="center">
  <img src="assets/theme-1.png" alt="Theme Preview 1" width="49%" />
  <img src="assets/theme-2.png" alt="Theme Preview 2" width="49%" />
  <br /><br />
  <img src="assets/theme-3.png" alt="Theme Preview 3" width="49%" />
  <img src="assets/theme-4.png" alt="Theme Preview 4" width="49%" />
</div>

## Quick Start with Docker

The fastest way to get MarkNote running is with Docker Compose:

```bash
# 1. Clone the repository
git clone https://github.com/odevsa/marknote.git
cd marknote

# 2. Start using Docker Compose
docker compose up -d
```

Open your browser at `http://localhost:3000`. On your first access, MarkNote will guide you through setting up your administrator credentials.


## Configuration

MarkNote is configured using environment variables:

| Variable | Default | Description |
| :--- | :--- | :--- |
| `PORT` | `3000` | Port on which the HTTP server listens |
| `HOST` | `0.0.0.0` | Host IP address binding |
| `DATA_DIR` | `./data/base` | Directory for SQLite metadata and FTS5 search database |
| `NOTES_DIR` | `./data/notes` | Directory where raw `.md` markdown files are stored |
| `JWT_SECRET` | *(auto-generated)* | Secret key used for signing authentication JWTs |
| `DATABASE_URL` | *(auto-configured)* | SQLite connection URI (`sqlite://./data/base/marknote.db?mode=rwc`) |


## Tech Stack

### Backend
- **Rust** & **Axum**: High-performance, memory-safe asynchronous web server.
- **SQLx** & **SQLite FTS5**: Embedded database for settings and full-text search index.
- **Notify**: Cross-platform filesystem watcher for real-time index synchronization.
- **Argon2** & **jsonwebtoken**: Industry-standard password hashing and authentication.

### Frontend
- **Svelte 5**: Modern reactive UI framework.
- **Tailwind CSS**: Utility-first styling with dynamic CSS design tokens.
- **CodeMirror 6**: Extensible code editor with custom Markdown syntax highlighting.
- **Marked** & **DOMPurify**: Fast Markdown parsing and secure HTML sanitization.


## Development

### Prerequisites
- **Rust** 1.80+ (`rustup default stable`)
- **Node.js** v20+ / v22+ & **npm**

### Running with Hot-Reload (Recommended)

Run both the Rust backend and Vite dev server simultaneously with one command from the project root:

```bash
npm run dev
```

This starts:
- Frontend Vite Server at `http://localhost:5173`
- Backend Axum Server at `http://localhost:3000` (proxied by Vite)

### Building for Production

```bash
# 1. Build Frontend Static Assets
cd frontend
npm install
npm run build

# 2. Compile Rust Single-Binary
cd ../backend
cargo build --release
```

The resulting binary in `backend/target/release/marknote` embeds all frontend assets into a standalone executable.
