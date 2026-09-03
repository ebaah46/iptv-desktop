use slint::ComponentHandle;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicI32, AtomicBool};
use libcore::facade::{ IptvFacade};
use slint::{Weak};

use crate::MainWindow;
use crate::HomeState;


/**
* Implement the home controller responsible for orchestrating the window
* communication between the UI layer and the core layer specifically for
* the home screen. Handles all that has to do with home screen callbacks and handlers
*/

pub struct HomeController {
    window: Weak<MainWindow>,
    core: Arc<IptvFacade>,
    loaded_channels: AtomicI32,
}

impl HomeController {

    const PAGE_SIZE: usize = 10;
    pub fn new(window: Weak<MainWindow>, core: Arc<IptvFacade>) -> Self {
        HomeController { window, core, loaded_channels: Default::default() }
    }

    /// Register the handlers in the home directory with the appropriate
    /// callbacks
    pub fn register(&self) {
        let Some(window) = self.window.upgrade() else {return};
        let home = window.global::<HomeState>();
        // home.on_channel_selected()
    }

    /// load feeds for selected channel
    fn load_feeds(&self, channel_id: String) {}

}