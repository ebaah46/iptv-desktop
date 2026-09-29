use std::sync::{Arc, RwLock};

use iced::{Element, Task, Theme};

use libcore::facade::{CoreFacade, IptvFacade};
use iced_video_player::Video;
use crate::ui::message::Message;
use crate::ui::page::Page;
use crate::ui::screens::home::{self, HomeMessage, HomeState};
use crate::ui::screens::player::{self, PlayerState};

/// Root application state.
pub struct App {
    pub facade: Arc<IptvFacade>,
    pub page: Page,
    pub home: HomeState,
    pub player: PlayerState,
}

impl App {
    pub fn new(facade: Arc<IptvFacade>, video_lock: Arc<RwLock<Option<Video>>>) -> Self {
        let mut player = PlayerState::default();
        player.video = video_lock;
        Self {
            facade,
            page: Page::Home,
            home: HomeState::new(),
            player,
        }
    }
}

/// Processes a message and mutates state.
pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::NavigateTo(page) => {
            state.page = page;
            Task::none()
        }
        Message::Home(msg) => {
            // Intercept ChannelClicked to navigate to Player screen.
            if let HomeMessage::ChannelClicked(id) = &msg {
                state.player.channel_id = id.clone();
                state.player.channel_name = id.clone();
                state.player.error_message.clear();
                state.page = Page::Player;
            }
            home::update(&mut state.home, msg, state.facade.clone())
                .map(Message::Home)
        }
        Message::Player(msg) => {
            let result = player::update(&mut state.player, msg, state.facade.clone());
            if state.player.is_back_requested {
                state.player.is_back_requested = false;
                state.page = Page::Home;
            }
            result.map(Message::Player)
        },
        Message::PrepareHome => {
            let facade = state.facade.clone();
            Task::perform(
                async move {
                    facade.refresh();
                },
                |_| Message::HomeReady,
            )
        },
        Message::HomeReady => {
            state.home.channel_data = state.facade.catalog_service.get_all();
            state.home.category_data =  state.facade.catalog_service.get_categories();
            state.home.countries_data = state.facade.catalog_service.get_countries();
            state.home.loading = false;
            Task::none()
        },
        Message::Unknown => Task::none(),
    }
}

/// Renders the current screen.
pub fn view(state: &App) -> Element<'_, Message> {
    match state.page {
        Page::Home => home::view(&state.home).map(Message::Home),
        Page::Player => {
            let video = state.player.loaded_video.as_ref();
            player::view(&state.player, video)
                .map(Message::Player)
        },
    }
}

/// Returns the active theme.
pub fn theme(_state: &App) -> Theme {
    Theme::Dark
}