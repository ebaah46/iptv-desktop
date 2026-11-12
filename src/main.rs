mod catalog;
mod config;
mod core;
mod player;
mod utils;

use config::AppConfig;

use libcore::facade::IptvFacade;
use libcore::services::{
    IpTvPlaybackController, IptvStreamResolver, catalog_repository::IptvCatalogRepository,
    catalog_service::IptvCatalogService,
};
use libcore_api::IptvRestClient;
use libcore_cache::{FileStore, IptvCacheStore};
use log::info;
use qtbridge::{QApp, QmlObject};
use std::sync::Arc;

use anyhow::Result as Res;
use tokio::runtime::Runtime;

use crate::core::AppState;
use crate::player::PlayerAdapter;
use crate::player::QmlPlayer;

fn main() -> Res<()> {
    // Set default log level to INFO so our [player], [app], [gst] logs show.
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    // ── 1. Qt must exist before any QObject is created ──────────────
    let mut app = QApp::new();

    // ── 2. Player QObject (Qt thread, before the event loop) ────────
    //     Created first because the adapter needs its QThreadHandle,
    //     and the facade needs the adapter.
    let qml_player = QmlPlayer::default_with_attached_qobject();
    let player_handle = qml_player.borrow().get_thread_handle();
    let player_adapter = Arc::new(PlayerAdapter::new(player_handle));
    let app_state = AppState::default_with_attached_qobject();

    // ── 3. Core services (synchronous) ──────────────────────────────
    let config = AppConfig::load()?;
    config.ensure_data_path();
    let cache_path = config.cache_db_path().expect("Cache path non-existent.");

    info!("Cache path: {:?}", cache_path);
    let client = Arc::new(IptvRestClient::new(config.api.base_url.clone()));
    let store = Arc::new(FileStore::new(cache_path, config.cache.ttl_days));
    let cache = Arc::new(IptvCacheStore::new(store));
    let repository = Arc::new(IptvCatalogRepository::new(client, cache));
    let catalog_service = Arc::new(IptvCatalogService::new(repository.clone()));
    let stream_resolver = Arc::new(IptvStreamResolver::new(repository));

    // ── 4. Runtime + facade ─────────────────────────────────────────
    let runtime = Runtime::new()?;

    let facade = runtime.block_on(async {
        let playback = Arc::new(IpTvPlaybackController::new(stream_resolver, player_adapter));
        // Wire the listener on the Qt thread, before the event loop starts.
        qml_player.borrow_mut().set_listener(playback.clone());
        Arc::new(IptvFacade::new(catalog_service, playback))
    });

    // ── 5. Publish the global context (must precede QML load) ───────
    core::context::init(facade, runtime);

    // ── 6. Hand the player to QML and run ───────────────────────────
    app.set_initial_object("player", qml_player)
        .set_initial_object("appState", app_state)
        .add_import_path("src/qml")
        .load_qml_from_file("src/qml/Main.qml")
        .run();

    Ok(())
}
