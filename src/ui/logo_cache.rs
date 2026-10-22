//! On-disk cache for downloaded channel logo bytes.
//!
//! Logos are keyed by a hash of their URL and stored as opaque raw bytes in a
//! dedicated `logos` directory under the application data path. On later runs
//! (or after re-visiting Home) the cached bytes are served instantly instead of
//! hitting the network again.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

/// A filesystem-backed cache for logo bytes.
#[derive(Debug, Clone)]
pub struct LogoCache {
    dir: Option<PathBuf>,
}

impl LogoCache {
    /// Creates a cache rooted at the given directory (or a no-op cache when
    /// `root` is `None`, e.g. when no platform data dir could be resolved).
    pub fn new(root: Option<PathBuf>) -> Self {
        let dir = root.map(|root| {
            let dir = root.join("logos");
            let _ = std::fs::create_dir_all(&dir);
            dir
        });
        Self { dir }
    }

    /// Returns the cached bytes for a logo URL, if present.
    pub fn get(&self, url: &str) -> Option<Vec<u8>> {
        if url.is_empty() {
            return None;
        }
        let path = self.path_for(url)?;
        std::fs::read(path).ok()
    }

    /// Stores the given bytes for a logo URL.
    pub fn put(&self, url: &str, bytes: &[u8]) {
        if url.is_empty() {
            return;
        }
        if let Some(path) = self.path_for(url) {
            let _ = std::fs::write(path, bytes);
        }
    }

    fn path_for(&self, url: &str) -> Option<PathBuf> {
        self.dir.as_ref().map(|dir| dir.join(format!("{}.img", Self::key(url))))
    }

    /// Deterministic hash of the URL used as the cache filename.
    fn key(url: &str) -> String {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }
}
