use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use walkdir::WalkDir;
use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub path: String,
    pub title: String,
    pub snippet: String,
}

/// Extracts a clean title from the markdown content (e.g. # Title) or fallback to file name.
fn extract_title(content: &str, file_name: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(h1) = trimmed.strip_prefix("# ") {
            return h1.trim().to_string();
        }
    }
    file_name.trim_end_matches(".md").to_string()
}

/// Indexes a single note into the SQLite FTS5 table
pub async fn index_note(pool: &SqlitePool, notes_dir: &Path, rel_path: &str) -> Result<(), AppError> {
    let full_path = notes_dir.join(rel_path);
    if !full_path.is_file() {
        return Ok(());
    }

    let content = match tokio::fs::read_to_string(&full_path).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };

    let file_name = Path::new(rel_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(rel_path);
    let title = extract_title(&content, file_name);

    // Remove existing entry for path if any, then insert
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM notes_fts WHERE path = ?")
        .bind(rel_path)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO notes_fts (path, title, content) VALUES (?, ?, ?)")
        .bind(rel_path)
        .bind(&title)
        .bind(&content)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

/// Removes a note from the FTS5 table
pub async fn remove_note(pool: &SqlitePool, rel_path: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM notes_fts WHERE path = ?")
        .bind(rel_path)
        .execute(pool)
        .await?;
    Ok(())
}

/// Initial synchronization: scans the entire notes_dir and indexes all markdown files into SQLite FTS5.
pub async fn sync_all_notes(pool: &SqlitePool, notes_dir: &Path) -> Result<(), AppError> {
    if !notes_dir.exists() {
        tokio::fs::create_dir_all(notes_dir).await?;
        return Ok(());
    }

    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM notes_fts").execute(&mut *tx).await?;

    for entry in WalkDir::new(notes_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext == "md" || ext == "markdown" || ext == "txt" {
            if let Ok(rel) = path.strip_prefix(notes_dir) {
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                if let Ok(content) = std::fs::read_to_string(path) {
                    let file_name = rel.file_name().and_then(|n| n.to_str()).unwrap_or(&rel_str);
                    let title = extract_title(&content, file_name);

                    sqlx::query("INSERT INTO notes_fts (path, title, content) VALUES (?, ?, ?)")
                        .bind(&rel_str)
                        .bind(&title)
                        .bind(&content)
                        .execute(&mut *tx)
                        .await?;
                }
            }
        }
    }

    tx.commit().await?;
    tracing::info!("FTS5 index synchronized successfully with notes directory.");
    Ok(())
}

/// Full-text search querying the FTS5 virtual table
pub async fn search_notes(pool: &SqlitePool, query: &str) -> Result<Vec<SearchResult>, AppError> {
    let clean_query = query
        .replace('"', "")
        .replace('*', "")
        .replace(':', "")
        .replace('^', "")
        .trim()
        .to_string();

    if clean_query.is_empty() {
        return Ok(Vec::new());
    }

    // Format query for prefix matching on terms
    let fts_query = clean_query
        .split_whitespace()
        .map(|w| format!("\"{}\"*", w))
        .collect::<Vec<_>>()
        .join(" ");

    let sql = r#"
        SELECT path, title, snippet(notes_fts, 2, '<mark>', '</mark>', '...', 15) as snippet
        FROM notes_fts
        WHERE notes_fts MATCH ?
        ORDER BY rank
        LIMIT 50
    "#;

    let rows: Vec<(String, String, String)> = sqlx::query_as(sql)
        .bind(&fts_query)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let results = rows
        .into_iter()
        .map(|(path, title, snippet)| SearchResult {
            path,
            title,
            snippet,
        })
        .collect();

    Ok(results)
}

/// Starts the file watcher in a background thread to update FTS5 when files change on disk
pub fn start_file_watcher(pool: SqlitePool, notes_dir: PathBuf) {
    let (tx, mut rx) = mpsc::channel::<PathBuf>(100);

    let watch_dir = notes_dir.clone();
    std::thread::spawn(move || {
        let (notify_tx, notify_rx) = std::sync::mpsc::channel();

        let mut watcher: RecommendedWatcher = match RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = notify_tx.send(event);
                }
            },
            notify::Config::default(),
        ) {
            Ok(w) => w,
            Err(e) => {
                tracing::error!("Failed to initialize file watcher: {}", e);
                return;
            }
        };

        if let Err(e) = watcher.watch(&watch_dir, RecursiveMode::Recursive) {
            tracing::error!("Failed to watch notes directory: {}", e);
            return;
        }

        while let Ok(event) = notify_rx.recv() {
            for path in event.paths {
                let _ = tx.blocking_send(path);
            }
        }
    });

    // Tokio async consumer to debounce and index
    tokio::spawn(async move {
        while let Some(path) = rx.recv().await {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if ext == "md" || ext == "markdown" || ext == "txt" {
                if let Ok(rel) = path.strip_prefix(&notes_dir) {
                    let rel_str = rel.to_string_lossy().replace('\\', "/");
                    if path.exists() {
                        let _ = index_note(&pool, &notes_dir, &rel_str).await;
                    } else {
                        let _ = remove_note(&pool, &rel_str).await;
                    }
                }
            }
        }
    });
}
