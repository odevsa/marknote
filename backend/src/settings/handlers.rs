use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{auth::jwt::AuthUser, error::AppError, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub show_recent_notes: bool,
    pub recent_notes_count: usize,
    pub auto_save: bool,
    pub auto_save_delay_ms: usize,
    pub enable_draft_recovery: bool,
}

pub async fn get_settings(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<AppSettings>, AppError> {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM settings")
        .fetch_all(&state.db)
        .await?;

    let mut show_recent_notes = true;
    let mut recent_notes_count = 5;
    let mut auto_save = true;
    let mut auto_save_delay_ms = 1500;
    let mut enable_draft_recovery = true;

    for (key, value) in rows {
        match key.as_str() {
            "show_recent_notes" => {
                show_recent_notes = value.parse::<bool>().unwrap_or(true);
            }
            "recent_notes_count" => {
                recent_notes_count = value.parse::<usize>().unwrap_or(5);
            }
            "auto_save" => {
                auto_save = value.parse::<bool>().unwrap_or(true);
            }
            "auto_save_delay_ms" => {
                auto_save_delay_ms = value.parse::<usize>().unwrap_or(1500);
            }
            "enable_draft_recovery" => {
                enable_draft_recovery = value.parse::<bool>().unwrap_or(true);
            }
            _ => {}
        }
    }

    Ok(Json(AppSettings {
        show_recent_notes,
        recent_notes_count,
        auto_save,
        auto_save_delay_ms,
        enable_draft_recovery,
    }))
}

pub async fn update_settings(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AppSettings>,
) -> Result<Json<AppSettings>, AppError> {
    let show_str = payload.show_recent_notes.to_string();
    let count_str = payload.recent_notes_count.to_string();
    let auto_save_str = payload.auto_save.to_string();
    let delay_str = payload.auto_save_delay_ms.to_string();
    let draft_str = payload.enable_draft_recovery.to_string();

    sqlx::query(
        "INSERT INTO settings (key, value) VALUES ('show_recent_notes', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
    )
    .bind(&show_str)
    .execute(&state.db)
    .await?;

    sqlx::query(
        "INSERT INTO settings (key, value) VALUES ('recent_notes_count', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
    )
    .bind(&count_str)
    .execute(&state.db)
    .await?;

    sqlx::query(
        "INSERT INTO settings (key, value) VALUES ('auto_save', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
    )
    .bind(&auto_save_str)
    .execute(&state.db)
    .await?;

    sqlx::query(
        "INSERT INTO settings (key, value) VALUES ('auto_save_delay_ms', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
    )
    .bind(&delay_str)
    .execute(&state.db)
    .await?;

    sqlx::query(
        "INSERT INTO settings (key, value) VALUES ('enable_draft_recovery', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
    )
    .bind(&draft_str)
    .execute(&state.db)
    .await?;

    Ok(Json(payload))
}
