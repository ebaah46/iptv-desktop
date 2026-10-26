use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;
use futures::{Stream, StreamExt};

use libcore_cache::FileStore;
use libcore_api::LogoHttpFetcher;
use libcore_cache::store::Store;
use crate::AppConfig;
use crate::ui_model::{ChannelItems, LogoImage};

/**
* This object provides the ability to load and cache logos
* retrieved for the given channels.
*/
pub struct LogoLoader {
    cache: FileStore,
    fetcher: LogoHttpFetcher,
}

impl LogoLoader {

    /// How long a cached logo stays valid, in days.
    const CACHE_TTL_DAYS: u8 = 100;
    /// Database file name used when no explicit cache path is provided.
    const CACHE_FILE_NAME: &'static str = "logos.redb";
    /// Key prefix so logo entries never collide with catalog entries.
    const CACHE_KEY_PREFIX: &'static str = "logo:";

    /// Confine raster logo images to a fixed size to avoid performance
    /// bottlenecks in iced_renderer
    const LOGO_TARGET_EDGE: u32 = 256;
    pub fn new(cache_path:Option<PathBuf> ) -> Self {
        let cfg = AppConfig::load().ok();
        let path = cache_path
            .or_else(|| {
                cfg.as_ref()
                    .and_then(AppConfig::ensure_data_path)
                    .map(|dir| dir.join(Self::CACHE_FILE_NAME))
            })
            .unwrap_or_else(|| std::env::temp_dir().join(Self::CACHE_FILE_NAME));
        Self {
            cache: FileStore::new(path, Self::CACHE_TTL_DAYS),
            fetcher: LogoHttpFetcher::new(),
        }
    }

    /// Loads and decodes the logo at `url` into a renderable image handle.
    /// Returns `None` when the url is empty, the download fails, or the bytes
    /// cannot be read from the cache.
    pub fn load(&self, url: &str, format: &str) -> Option<LogoImage> {
        self.get(url).map(|bytes| Self::decode(format, bytes))
    }

    /// Returns the logo bytes for `url`, serving them from the cache when
    /// possible and fetching (then caching) them on a miss.
    fn get(&self, url: &str) -> Option<Vec<u8>> {
        if url.is_empty() {
            return None;
        }

        let key = Self::key(url);
        if let Some(cached) = self.cache.get_item::<Vec<u8>>(&key) {
            return Some(cached);
        }

        let bytes = self.fetcher.fetch(url).ok()?;
        let _ = self.cache.save_item(&key, &bytes);
        Some(bytes)
    }

    /// Deterministic hash of the URL used as the cache filename.
    fn key(url: &str) -> String {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    /// Drops the cached entry for `url` .
    pub fn evict(&self, url: &str) {
        if !url.is_empty() {
            self.cache.evict(&Self::key(url));
        }
    }

    /// Wraps raw image bytes in the handle matching the source format.
    fn decode(format: &str, bytes: Vec<u8>) -> LogoImage {
        if format.eq_ignore_ascii_case("svg") {
            LogoImage::Svg(iced::widget::svg::Handle::from_memory(bytes))
        } else {
            match Self::decode_scaled(&bytes) {
                Some(handle) => LogoImage::Raster(handle),
                // leave to iced renderer.
                None => LogoImage::Raster(iced::widget::image::Handle::from_bytes(bytes)),
            }
        }
    }

    /// Decodes `bytes` and scales it down to at most [`LOGO_TARGET_EDGE`] px,
    /// returning a raw-RGBA handle so the renderer performs no decoding.
    fn decode_scaled(bytes: &[u8]) -> Option<iced::widget::image::Handle> {
        use image::imageops::FilterType;

        let image = image::load_from_memory(bytes).ok()?;
        let (width, height) = (image.width(), image.height());
        let longest = width.max(height);

        let image = if longest > Self::LOGO_TARGET_EDGE {
            let scale = Self::LOGO_TARGET_EDGE as f32 / longest as f32;
            let target_w = ((width as f32 * scale).round() as u32).max(1);
            let target_h = ((height as f32 * scale).round() as u32).max(1);
            image.resize_exact(target_w, target_h, FilterType::Triangle)
        } else {
            // leave smaller images to avoid pixelation
            image
        };

        let rgba = image.to_rgba8();
        let (w, h) = (rgba.width(), rgba.height());
        Some(iced::widget::image::Handle::from_rgba(w, h, rgba.into_raw()))
    }
}


/// Number of logo loads allowed to run in parallel.
const CONCURRENCY: usize = 8;
/// Number of finished loads grouped into a single progressive update.
const BATCH_SIZE: usize = 12;

/// A batch of loaded logos keyed by channel id; `None` marks a failed load.
pub type LogoBatch = Vec<(String, Option<LogoImage>)>;


/// Streams decoded logos for every item carrying a logo url, in progressive
/// batches so the grid fills in as loads complete.
///
/// Each load runs on a blocking thread (the cache and HTTP client are
/// synchronous), bounded by [`CONCURRENCY`], against one shared loader.
pub fn load_all(
    loader: Arc<LogoLoader>,
    items: ChannelItems,
) -> impl Stream<Item = LogoBatch> {
    futures::stream::iter(items)
        .filter(|item| futures::future::ready(item.has_logo()))
        .map(move |item| {
            let loader = loader.clone();
            tokio::task::spawn_blocking(move || {
                let image = loader.load(item.logo_url(), item.logo_format());
                (item.id().to_string(), image)
            })
        })
        .buffer_unordered(CONCURRENCY)
        .map(|res| res.unwrap_or_else(|_| (String::new(), None)))
        .ready_chunks(BATCH_SIZE)
}


/// Merges a loaded batch into a channel list by channel id.
pub fn merge_logo_batch(batch: &LogoBatch, items: &ChannelItems) {
    let mut pending: HashMap<&str, &LogoImage> = batch.iter()
        .filter_map(|(id, image)| image.as_ref().map(|image| (id.as_str(), image))).collect();

    if pending.is_empty() {
        return;
    }

    for item in items.iter() {
        if let Some(image) = pending.remove(item.id()){
            item.set_logo_handle(image.clone());
            if pending.is_empty(){
                break;
            }
        }
    }

}