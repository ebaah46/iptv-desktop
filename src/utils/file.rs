use std::path::PathBuf;
use crate::config::AppConfig;

/// Returns the application's runtime data directory path.
///
/// This is where the application manages its runtime files
/// (caches, logs, user-managed files). The path is derived
/// from the application identity in the config file.
///
/// On success the directory is guaranteed to exist.
pub fn get_data_path(config: &AppConfig) -> PathBuf {
    config
        .ensure_data_path()
        .expect("Failed to determine or create application data directory")
}