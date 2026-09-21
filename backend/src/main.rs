use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod db;
mod error;
mod notes;
mod routes;
mod settings;

pub struct AppState {
    pub config: config::Config,
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "marknote=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env();
    tracing::info!("Starting MarkNote with config: {:?}", config);

    if !config.data_dir.exists() {
        tokio::fs::create_dir_all(&config.data_dir).await?;
    }
    if !config.notes_dir.exists() {
        tokio::fs::create_dir_all(&config.notes_dir).await?;
    }

    let pool = db::init_db(&config.database_url).await?;
    tracing::info!("Connected to SQLite database at {}", config.database_url);

    if let Err(e) = notes::watcher::sync_all_notes(&pool, &config.notes_dir).await {
        tracing::warn!("Failed initial FTS5 synchronization: {}", e);
    }

    notes::watcher::start_file_watcher(pool.clone(), config.notes_dir.clone());

    let state = Arc::new(AppState {
        config: config.clone(),
        db: pool,
    });

    let app = routes::create_router(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("MarkNote server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

