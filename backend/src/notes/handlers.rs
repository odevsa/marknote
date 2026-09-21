use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    auth::jwt::AuthUser,
    error::AppError,
    notes::{
        fs_service::{self, FileTreeNode},
        watcher::{self, SearchResult},
    },
    AppState,
};

#[derive(Deserialize)]
pub struct PathQuery {
    pub path: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct NoteContentResponse {
    pub path: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct SaveNoteRequest {
    pub path: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub path: String,
    pub is_dir: bool,
}

#[derive(Deserialize)]
pub struct RenameItemRequest {
    pub old_path: String,
    pub new_path: String,
}

#[derive(Deserialize)]
pub struct DeleteItemRequest {
    pub path: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

pub async fn get_tree(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FileTreeNode>>, AppError> {
    let tree = fs_service::get_tree(&state.config.notes_dir).await?;
    Ok(Json(tree))
}

pub async fn get_content(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<PathQuery>,
) -> Result<Json<NoteContentResponse>, AppError> {
    let content = fs_service::read_note(&state.config.notes_dir, &query.path).await?;
    Ok(Json(NoteContentResponse {
        path: query.path,
        content,
    }))
}

pub async fn save_content(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SaveNoteRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    fs_service::write_note(&state.config.notes_dir, &payload.path, &payload.content).await?;
    let _ = watcher::index_note(&state.db, &state.config.notes_dir, &payload.path).await;

    Ok(Json(SuccessResponse { success: true }))
}

pub async fn create_item(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    fs_service::create_item(&state.config.notes_dir, &payload.path, payload.is_dir).await?;

    if !payload.is_dir {
        let _ = watcher::index_note(&state.db, &state.config.notes_dir, &payload.path).await;
    }

    Ok(Json(SuccessResponse { success: true }))
}

pub async fn rename_item(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RenameItemRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    fs_service::rename_item(&state.config.notes_dir, &payload.old_path, &payload.new_path).await?;

    let _ = watcher::remove_note(&state.db, &payload.old_path).await;
    let _ = watcher::index_note(&state.db, &state.config.notes_dir, &payload.new_path).await;

    Ok(Json(SuccessResponse { success: true }))
}

pub async fn delete_item(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteItemRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    fs_service::delete_item(&state.config.notes_dir, &payload.path).await?;
    let _ = watcher::remove_note(&state.db, &payload.path).await;

    Ok(Json(SuccessResponse { success: true }))
}

pub async fn search_notes(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<SearchResult>>, AppError> {
    let results = watcher::search_notes(&state.db, &query.q).await?;
    Ok(Json(results))
}

#[derive(Deserialize)]
pub struct RecentQuery {
    pub limit: Option<usize>,
}

pub async fn get_recent_notes(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<RecentQuery>,
) -> Result<Json<Vec<fs_service::RecentNoteItem>>, AppError> {
    let limit = query.limit.unwrap_or(5);
    let items = fs_service::get_recent_notes(&state.config.notes_dir, limit).await?;
    Ok(Json(items))
}


