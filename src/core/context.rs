use std::sync::{Arc, OnceLock};

use libcore::{
    facade::{CoreFacade, IptvFacade},
    services::{CatalogService, PlaybackController},
};
use tokio::runtime::{Handle, Runtime};

/// Application-wide services, initialised once before QML loads.
/// Also provides access to tokio runtime and the application wide 
/// access to the core library entrypoint. Note that the application 
/// context is a singleton
pub struct AppContext {
    facade: Arc<IptvFacade>,
    runtime: Runtime,
}

impl AppContext {
    pub fn new(facade: Arc<IptvFacade>, runtime: Runtime) -> Self {
        Self { facade, runtime }
    }

    // ── Core services ────────────────────────────────────────────
    pub fn catalog(&self) -> Arc<dyn CatalogService> {
        self.facade.catalog_service.clone()
    }

    pub fn playback(&self) -> Arc<dyn PlaybackController> {
        self.facade.playback_controller.clone()
    }

    // ── Async helpers ────────────────────────────────────────────
    /// Spawn async work on the tokio worker pool.
    pub fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(future);
    }

    /// Run blocking work on the tokio blocking pool.
    pub fn spawn_blocking<F, R>(&self, f: F) -> tokio::task::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.runtime.spawn_blocking(f)
    }

    /// A cheap clone of the runtime handle, for APIs that need one.
    pub fn handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    pub fn refresh(&self) {
        self.facade.refresh();
    }
}

static CONTEXT: OnceLock<AppContext> = OnceLock::new();

/// Initialise the global context. Must run before QML loads.
pub fn init(facade: Arc<IptvFacade>, runtime: Runtime) {
    CONTEXT
        .set(AppContext::new(facade, runtime))
        .ok()
        .expect("AppContext already initialised");
}

/// Access the global context.
pub fn get() -> &'static AppContext {
    CONTEXT
        .get()
        .expect("context::init must run before QML loads")
}
