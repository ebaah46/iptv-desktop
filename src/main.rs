use std::sync::Arc;
use slint::ComponentHandle;
use libcore::facade::IptvFacade;
use libcore_api::IptvRestClient;
use libcore_cache::{FileStore, IptvCacheStore};
use libcore::services::{catalog_repository::IptvCatalogRepository, catalog_service::IptvCatalogService, CatalogRepository, IpTvPlaybackController, IptvStreamResolver};
use iptv_desktop::{HomeController, AppConfig, GstPlayerController, MainWindow};
fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let config = AppConfig::load().expect("Failed to load config");

    let client = Arc::new(IptvRestClient::new(config.api.base_url.clone()));
    let cache_path = config.cache_db_path()
        .expect("Failed to determine cache database path");
    println!("Cache path: {:?}", cache_path);
    let store = Arc::new(FileStore::new(cache_path, config.cache.ttl_days));
    let cache = Arc::new(IptvCacheStore::new(store));
    let iptv_repository = Arc::new(IptvCatalogRepository::new(client, cache));
    let iptv_service = Arc::new(IptvCatalogService::new(iptv_repository.clone()));
    let player = Arc::new(GstPlayerController::new());
    let stream_resolver = Arc::new(IptvStreamResolver::new(iptv_repository.clone()));
    let play_back_controller = Arc::new(IpTvPlaybackController::new(stream_resolver, player));
    let facade = Arc::new(IptvFacade::new(iptv_service, play_back_controller));
    let home_controller = HomeController::new(main_window.as_weak(), facade.clone());
    // load catalog data before starting
    iptv_repository.refresh();
    home_controller.register();
    main_window.run()
}