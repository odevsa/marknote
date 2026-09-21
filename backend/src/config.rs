use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub data_dir: PathBuf,
    pub notes_dir: PathBuf,
    pub jwt_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let data_dir = PathBuf::from(
            std::env::var("DATA_DIR").unwrap_or_else(|_| "./data/base".to_string()),
        );
        let notes_dir = PathBuf::from(
            std::env::var("NOTES_DIR").unwrap_or_else(|_| "./data/notes".to_string()),
        );

        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            "marknote_default_secret_key_change_in_production_32bytes_long".to_string()
        });

        let db_path = data_dir.join("marknote.db");
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| format!("sqlite://{}?mode=rwc", db_path.to_string_lossy()));

        Self {
            host,
            port,
            data_dir,
            notes_dir,
            jwt_secret,
            database_url,
        }
    }
}

