use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
use tokio::fs;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String, // Relative path from notes root, using forward slashes
    pub is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FileTreeNode>>,
}

/// Safely resolves a relative path within the notes base directory,
/// preventing any directory traversal attacks (e.g., ../).
pub fn resolve_safe_path(base_dir: &Path, rel_path: &str) -> Result<PathBuf, AppError> {
    let clean_rel = rel_path.trim().trim_start_matches('/');
    if clean_rel.is_empty() {
        return Ok(base_dir.to_path_buf());
    }

    let rel_path_buf = Path::new(clean_rel);

    // Ensure no component is ParentDir (..) or Prefix
    for component in rel_path_buf.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            _ => {
                return Err(AppError::BadRequest(
                    "Invalid path: directory traversal or absolute paths are forbidden".to_string(),
                ));
            }
        }
    }

    let full_path = base_dir.join(rel_path_buf);
    Ok(full_path)
}

/// Recursively reads the notes directory and returns a sorted tree hierarchy.
pub async fn get_tree(base_dir: &Path) -> Result<Vec<FileTreeNode>, AppError> {
    if !base_dir.exists() {
        fs::create_dir_all(base_dir).await?;
    }

    read_dir_recursive(base_dir, "").await
}

fn read_dir_recursive<'a>(
    base_dir: &'a Path,
    current_rel: &'a str,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<FileTreeNode>, AppError>> + Send + 'a>> {
    Box::pin(async move {
        let current_dir = if current_rel.is_empty() {
            base_dir.to_path_buf()
        } else {
            base_dir.join(current_rel)
        };

        let mut entries = fs::read_dir(&current_dir).await?;
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let file_name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files/directories (like .git, .DS_Store)
            if file_name.starts_with('.') {
                continue;
            }

            let file_type = entry.file_type().await?;
            let item_rel = if current_rel.is_empty() {
                file_name.clone()
            } else {
                format!("{}/{}", current_rel, file_name)
            };

            if file_type.is_dir() {
                let children = read_dir_recursive(base_dir, &item_rel).await?;
                dirs.push(FileTreeNode {
                    name: file_name,
                    path: item_rel,
                    is_dir: true,
                    children: Some(children),
                });
            } else if file_name.ends_with(".md") || file_name.ends_with(".markdown") || file_name.ends_with(".txt") {
                files.push(FileTreeNode {
                    name: file_name,
                    path: item_rel,
                    is_dir: false,
                    children: None,
                });
            }
        }

        dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        dirs.extend(files);
        Ok(dirs)
    })
}

pub async fn read_note(base_dir: &Path, rel_path: &str) -> Result<String, AppError> {
    let full_path = resolve_safe_path(base_dir, rel_path)?;
    if !full_path.is_file() {
        return Err(AppError::NotFound(format!("Note not found: {}", rel_path)));
    }

    let content = fs::read_to_string(&full_path).await?;
    Ok(content)
}

pub async fn write_note(base_dir: &Path, rel_path: &str, content: &str) -> Result<(), AppError> {
    let full_path = resolve_safe_path(base_dir, rel_path)?;

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(&full_path, content).await?;
    Ok(())
}

pub async fn create_item(base_dir: &Path, rel_path: &str, is_dir: bool) -> Result<(), AppError> {
    let full_path = resolve_safe_path(base_dir, rel_path)?;

    if full_path.exists() {
        return Err(AppError::BadRequest(format!(
            "Item already exists: {}",
            rel_path
        )));
    }

    if is_dir {
        fs::create_dir_all(&full_path).await?;
    } else {
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&full_path, "").await?;
    }

    Ok(())
}

pub async fn rename_item(base_dir: &Path, old_rel: &str, new_rel: &str) -> Result<(), AppError> {
    let old_path = resolve_safe_path(base_dir, old_rel)?;
    let new_path = resolve_safe_path(base_dir, new_rel)?;

    if !old_path.exists() {
        return Err(AppError::NotFound(format!("Item not found: {}", old_rel)));
    }
    if new_path.exists() {
        return Err(AppError::BadRequest(format!(
            "Target path already exists: {}",
            new_rel
        )));
    }

    if let Some(parent) = new_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::rename(&old_path, &new_path).await?;
    Ok(())
}

pub async fn delete_item(base_dir: &Path, rel_path: &str) -> Result<(), AppError> {
    let full_path = resolve_safe_path(base_dir, rel_path)?;

    if !full_path.exists() {
        return Err(AppError::NotFound(format!("Item not found: {}", rel_path)));
    }

    if full_path.is_dir() {
        fs::remove_dir_all(&full_path).await?;
    } else {
        fs::remove_file(&full_path).await?;
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentNoteItem {
    pub path: String,
    pub title: String,
    pub content: String,
    pub modified_at: String,
}

pub fn format_note_title(rel_path: &str) -> String {
    if let Some(stripped) = rel_path.strip_suffix(".md") {
        stripped.to_string()
    } else if let Some(stripped) = rel_path.strip_suffix(".markdown") {
        stripped.to_string()
    } else if let Some(stripped) = rel_path.strip_suffix(".txt") {
        stripped.to_string()
    } else {
        rel_path.to_string()
    }
}

pub async fn get_recent_notes(base_dir: &Path, limit: usize) -> Result<Vec<RecentNoteItem>, AppError> {
    if !base_dir.exists() {
        return Ok(Vec::new());
    }

    struct TempNote {
        path: String,
        title: String,
        content: String,
        mtime: std::time::SystemTime,
    }

    let mut all_notes: Vec<TempNote> = Vec::new();

    fn collect_files<'a>(
        base_dir: &'a Path,
        current_rel: &'a str,
        acc: &'a mut Vec<TempNote>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + 'a>> {
        Box::pin(async move {
            let current_dir = if current_rel.is_empty() {
                base_dir.to_path_buf()
            } else {
                base_dir.join(current_rel)
            };

            let mut entries = match fs::read_dir(&current_dir).await {
                Ok(e) => e,
                Err(_) => return Ok(()),
            };

            while let Some(entry) = entries.next_entry().await? {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.starts_with('.') {
                    continue;
                }

                let file_type = entry.file_type().await?;
                let item_rel = if current_rel.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", current_rel, file_name)
                };

                if file_type.is_dir() {
                    collect_files(base_dir, &item_rel, acc).await?;
                } else if file_name.ends_with(".md")
                    || file_name.ends_with(".markdown")
                    || file_name.ends_with(".txt")
                {
                    let metadata = entry.metadata().await?;
                    let mtime = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    let full_path = base_dir.join(&item_rel);
                    let content = fs::read_to_string(&full_path).await.unwrap_or_default();
                    let title = format_note_title(&item_rel);

                    acc.push(TempNote {
                        path: item_rel,
                        title,
                        content,
                        mtime,
                    });
                }
            }

            Ok(())
        })
    }

    collect_files(base_dir, "", &mut all_notes).await?;

    all_notes.sort_by(|a, b| b.mtime.cmp(&a.mtime));

    let recent = all_notes
        .into_iter()
        .take(limit)
        .map(|n| {
            let datetime: chrono::DateTime<chrono::Utc> = n.mtime.into();
            RecentNoteItem {
                path: n.path,
                title: n.title,
                content: n.content,
                modified_at: datetime.to_rfc3339(),
            }
        })
        .collect();

    Ok(recent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_path_traversal_prevention() {
        let base = PathBuf::from("/tmp/marknote_notes");

        // Legitimate paths
        assert!(resolve_safe_path(&base, "note.md").is_ok());
        assert!(resolve_safe_path(&base, "folder/subfolder/note.md").is_ok());

        // Dangerous paths with directory traversal
        assert!(resolve_safe_path(&base, "../secret.txt").is_err());
        assert!(resolve_safe_path(&base, "folder/../../etc/passwd").is_err());
        assert!(resolve_safe_path(&base, "..").is_err());
    }
}
