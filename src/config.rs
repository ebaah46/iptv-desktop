use serde::Deserialize;

/// Top-level application configuration.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppIdentity,
    pub api: ApiConfig,
    pub cache: CacheConfig,
}

/// Application identity used to derive platform-specific directories
/// (e.g. data dir, config dir) via the `directories` crate.
#[derive(Debug, Deserialize)]
pub struct AppIdentity {
    pub qualifier: String,
    pub organization: String,
    pub application: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    pub ttl_days: u8,
    pub file_path: Option<String>,
    pub db_file: String,
}

impl AppConfig {
    /// Loads the application configuration.
    ///
    /// Resolution order:
    /// 1. If the `IPTV_CONFIG_PATH` environment variable is set, load from that path.
    /// 2. Otherwise, fall back to the embedded default config (`config.default.json`).
    pub fn load() -> Result<Self, ConfigError> {
        // Check for environment variable override first.
        if let Ok(path) = std::env::var("IPTV_CONFIG_PATH") {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| ConfigError::Io(e.to_string()))?;
            let config: AppConfig = serde_json::from_str(&content)
                .map_err(|e| ConfigError::Parse(e))?;
            return Ok(config);
        }

        // Fall back to the embedded default config.
        let content = include_str!("../config.default.json");
        let config: AppConfig = serde_json::from_str(content)
            .map_err(|e| ConfigError::Parse(e))?;
        Ok(config)
    }

    /// Returns the runtime data directory for this application.
    ///
    /// This is where the application stores runtime-managed files
    /// (caches, logs, user data). It is **not** the same as the
    /// config file location — config is read-only and deployed
    /// with the app bundle.
    pub fn data_path(&self) -> Option<std::path::PathBuf> {
        directories::ProjectDirs::from(
            &self.app.qualifier,
            &self.app.organization,
            &self.app.application,
        )
        .map(|d| d.data_dir().to_path_buf())
    }

    /// Ensures the runtime data directory exists and returns its path.
    pub fn ensure_data_path(&self) -> Option<std::path::PathBuf> {
        let path = self.data_path()?;
        std::fs::create_dir_all(&path).ok()?;
        Some(path)
    }

    /// Returns the full path to the redb database file,
    /// joining the runtime data directory with the configured `db_file` name.
    pub fn cache_db_path(&self) -> Option<std::path::PathBuf> {
        self.data_path().map(|p| p.join(&self.cache.db_file))
    }
}

/// Errors that can occur while loading the configuration.
#[derive(Debug)]
pub enum ConfigError {
    Io(String),
    Parse(serde_json::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(msg) => write!(f, "IO error: {}", msg),
            ConfigError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}