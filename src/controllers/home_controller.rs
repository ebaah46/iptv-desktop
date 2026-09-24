use slint::{ComponentHandle, ModelRc, VecModel};
use std::sync::Arc;
use libcore::facade::IptvFacade;
use slint::Weak;

use crate::{Category, Channel, Country, MainWindow};
use crate::HomeState;
use crate::NavigationState;
use crate::Page;
use crate::PlayerState;
use crate::mappers::{country_mapper::to_country, category_mapper::to_category, channel_mapper::to_channel_info};

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

    pub fn new(window: Weak<MainWindow>, core: Arc<IptvFacade>) -> Self {
        HomeController { window, core }
    }

    /// Register the handlers in the home directory with the appropriate
    /// callbacks
    pub fn register(&self) {
        let Some(window) = self.window.upgrade() else {return};
        let home = window.global::<HomeState>();

        let weak_window = self.window.clone();
        home.on_channel_selected(move |channel_id| {
            if let Some(window) = weak_window.upgrade() {
                let nav = window.global::<NavigationState>();
                let player = window.global::<PlayerState>();

                // Set channel ID for the player screen
                nav.set_selected_channel_id(channel_id.clone());

                // Notify the player to load the stream
                player.set_is_loading(true);
                player.set_error_message("".into());

                // Navigate to player page
                nav.set_current_page(Page::Player);

                // Trigger stream loading in the player controller
                player.invoke_stream_requested(channel_id);
            }
        });

        let weak_window = self.window.clone();
        let core = self.core.clone();
        home.on_category_selected(move |category| {
            if let Some(window) = weak_window.upgrade() {
                let home = window.global::<HomeState>();
                Self::handle_category_selected(&core.clone(), &home, category);
            }
        });
        let countries: VecModel<Country> = self.core.catalog_service.get_countries().into_iter().map(to_country).collect();
        home.set_countries(ModelRc::new(countries));
        let categories:VecModel<Category> = self.core.catalog_service.get_categories().into_iter().map(to_category).collect();
        home.set_categories(ModelRc::new(categories));
        let channels:VecModel<Channel> = self.core.catalog_service.get_all().into_iter().map(to_channel_info).collect();
        home.set_channels(ModelRc::new(channels));
        dbg!("HomeController registered successfully");
    }

    /// Handle category selection: filter channels by the selected category
    fn handle_category_selected(core: &Arc<IptvFacade>, home: &HomeState, category: slint::SharedString) {
        let _ = (core, home, category);
        // TODO: Implement category filtering once the data model is updated
    }
}