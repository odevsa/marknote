use axum::{
    body::Body,
    http::{header, HeaderValue, Response, StatusCode, Uri},
    routing::{delete, get, post},
    Router,
};
use rust_embed::RustEmbed;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::{
    auth::handlers::{auth_status, login, logout, me, setup},
    notes::handlers::{
        create_item, delete_item, get_content, get_recent_notes, get_tree, rename_item, save_content, search_notes,
    },
    settings::handlers::{get_settings, update_settings},
    AppState,
};

#[derive(RustEmbed)]
#[folder = "../frontend/build/"]
pub struct FrontendAssets;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        // Auth routes
        .route("/auth/status", get(auth_status))
        .route("/auth/setup", post(setup))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        // Notes routes
        .route("/notes/tree", get(get_tree))
        .route("/notes/recent", get(get_recent_notes))
        .route("/notes/content", get(get_content).post(save_content))
        .route("/notes/create", post(create_item))
        .route("/notes/rename", post(rename_item))
        .route("/notes", delete(delete_item))
        .route("/notes/search", get(search_notes))
        // Settings routes
        .route("/settings", get(get_settings).post(update_settings));

    Router::new()
        .nest("/api", api_routes)
        .fallback(static_or_spa_handler)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

async fn static_or_spa_handler(uri: Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');

    // 1. Try exact asset
    if !path.is_empty() {
        if let Some(content) = FrontendAssets::get(path) {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            return Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).unwrap())
                .body(Body::from(content.data))
                .unwrap();
        }
    }

    // 2. Fallback to index.html (SPA routing)
    if let Some(content) = FrontendAssets::get("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8"))
            .body(Body::from(content.data))
            .unwrap();
    }

    // If frontend hasn't been built or asset not found
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("MarkNote frontend asset not found. Build the frontend first."))
        .unwrap()
}
