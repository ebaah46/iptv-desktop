use slint::ComponentHandle;
use std::sync::Arc;
use libcore::facade::{CoreFacade, IptvFacade};
use slint::Weak;

use crate::MainWindow;
use crate::HomeState;
use crate::NavigationState;
use crate::Page;

/**
* Implement the home controller responsible for orchestrating the window
* communication between the UI layer and the core layer specifically for
* the home screen. Handles all that has to do with home screen callbacks and handlers
*/

pub struct HomeController {
    window: Weak<MainWindow>,
    core: Arc<IptvFacade>,
}

impl HomeController {

    const PAGE_SIZE: usize = 10;
    pub fn new(window: Weak<MainWindow>, core: Arc<IptvFacade>) -> Self {
        HomeController { window, core }
    }

    /// Register the handlers in the home directory with the appropriate
    /// callbacks
    pub fn register(&self) {
        let Some(window) = self.window.upgrade() else {return};
        let home = window.global::<HomeState>();

        let weak_window = self.window.clone();
        home.on_channel_selected(move |channel_name| {
            if let Some(window) = weak_window.upgrade() {
                Self::handle_channel_selected(&window, channel_name);
            }
        });

        let weak_window = self.window.clone();
        let core = self.core.clone();
        home.on_category_selected(move |category| {
            if let Some(window) = weak_window.upgrade() {
                let home = window.global::<HomeState>();
                Self::handle_category_selected(&core, &home, category);
            }
        });
    }

    /// Handle channel selection: navigate to the player screen
    fn handle_channel_selected(window: &MainWindow, channel_name: slint::SharedString) {
        let nav = window.global::<NavigationState>();
        nav.set_selected_channel_name(channel_name);
        nav.set_current_page(Page::Player);
    }

    /// Handle category selection: filter channels by the selected category
    fn handle_category_selected(core: &Arc<IptvFacade>, home: &HomeState, category: slint::SharedString) {
        let _ = (core, home, category);
        // TODO: Implement category filtering once the data model is updated
    }

    /// load feeds for selected channel
    fn load_feeds(&self, _channel_id: String) {}

}