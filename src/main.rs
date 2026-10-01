pub mod config;
pub mod utils;
pub mod player;
mod ui;

pub use config::AppConfig;
pub use player::GstPlayerController;

use std::sync::Arc;
use std::time::Duration;
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
use tokio::sync::mpsc;
use crate::player::commands::{PlayerCommand, PlayerEvent};

fn main() -> Res<()> {
    // Set default log level to INFO so our [player], [app], [gst] logs show.
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    // ── Synchronous initialization (outside tokio runtime) ──────────
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
    let stream_resolver = Arc::new(IptvStreamResolver::new(iptv_repository));

    // ── Runtime-dependent initialization (inside tokio runtime) ─────
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let (facade, command_rx, event_tx) = rt.block_on(async {
        let (command_tx, command_rx) = mpsc::unbounded_channel::<PlayerCommand>();
        let (event_tx, event_rx) = mpsc::unbounded_channel::<PlayerEvent>();
        let player = Arc::new(GstPlayerController::new(command_tx, event_rx));
        let play_back_controller = Arc::new(IpTvPlaybackController::new(stream_resolver, player.clone()));
        player.set_listener(play_back_controller.clone());
        let facade = Arc::new(IptvFacade::new(iptv_service, play_back_controller));
        (facade, command_rx, event_tx)
    });
    let _ = ui::run(facade, command_rx, event_tx);
    Ok(())
}