pub mod config;
pub mod utils;
pub mod player;
mod ui;

pub use config::AppConfig;
pub use player::GstPlayerController;

use std::sync::Arc;
use libcore::facade::IptvFacade;
use libcore_api::IptvRestClient;
use libcore_cache::{FileStore, IptvCacheStore};
use libcore::services::{
    catalog_repository::IptvCatalogRepository,
    catalog_service::IptvCatalogService,
    IpTvPlaybackController, IptvStreamResolver,
};

use anyhow::Result as Res;

fn main() -> Res<()> {
    let facade = Arc::new(build_facade());
    ui::run(facade)?;
    Ok(())
}

fn build_facade() -> IptvFacade {
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
    let player = Arc::new(GstPlayerController::new());
    let stream_resolver = Arc::new(IptvStreamResolver::new(iptv_repository.clone()));
    let play_back_controller = Arc::new(IpTvPlaybackController::new(stream_resolver, player));
    IptvFacade::new(iptv_service, play_back_controller)
}