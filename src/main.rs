pub mod config;
pub mod utils;
pub mod player;
mod ui;

pub use config::AppConfig;
pub use player::GstPlayerController;

use std::sync::{Arc, RwLock};
use libcore::facade::IptvFacade;
use libcore_api::IptvRestClient;
use libcore_cache::{FileStore, IptvCacheStore};
use libcore::services::{
    catalog_repository::IptvCatalogRepository,
    catalog_service::IptvCatalogService,
    IpTvPlaybackController, IptvStreamResolver,
};

use anyhow::Result as Res;
use tokio::runtime::Runtime;

fn main() -> Res<()> {
    // Set default log level to INFO so our [player], [app], [gst] logs show.
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let (facade, player) = build_facade();
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(async {
        let _ = ui::run(facade, player);
        Ok(())
    })
}

fn build_facade() -> (Arc<IptvFacade>, Arc<GstPlayerController>) {
    let config = AppConfig::load().expect("Failed to load config");

    let client = Arc::new(IptvRestClient::new(config.api.base_url.clone()));
    let cache_path = config
        .cache_db_path()
        .expect("Failed to determine cache database path");
    println!("Cache path: {:?}", cache_path);
    let store = Arc::new(FileStore::new(cache_path, config.cache.ttl_days));
    let cache = Arc::new(IptvCacheStore::new(store));
    let iptv_repository = Arc::new(IptvCatalogRepository::new(client, cache));
    let iptv_service = Arc::new(IptvCatalogService::new(iptv_repository.clone()));
    let video_lock = Arc::new(RwLock::new(None));
    let player = Arc::new(GstPlayerController::new(video_lock));
    let stream_resolver = Arc::new(IptvStreamResolver::new(iptv_repository.clone()));
    let play_back_controller = Arc::new(IpTvPlaybackController::new(stream_resolver, player.clone()));
    let facade = Arc::new(IptvFacade::new(iptv_service, play_back_controller));
    (facade, player)
}