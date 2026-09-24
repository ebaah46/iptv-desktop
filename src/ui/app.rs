use std::sync::Arc;

use iced::{Element, Task, Theme};

use libcore::facade::{CoreFacade, IptvFacade};

use crate::ui::message::Message;
use crate::ui::page::Page;
use crate::ui::screens::home::{self, HomeState};
use crate::ui::screens::player::{self, PlayerState};

/// Root application state.
pub struct App {
    pub facade: Arc<IptvFacade>,
    pub page: Page,
    pub home: HomeState,
    pub player: PlayerState,
}

impl App {
    pub fn new(facade: Arc<IptvFacade>) -> Self {
        Self {
            facade,
            page: Page::Home,
            home: HomeState::new(),
            player: PlayerState::default(),
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
            home::update(&mut state.home, msg, state.facade.clone()).map(Message::Home)
        }
        Message::Player(msg) => {
            let is_back = matches!(&msg, player::PlayerMessage::BackRequested);
            let is_channel_selected = matches!(&msg, player::PlayerMessage::ChannelSelected(_));
            let result = player::update(&mut state.player, msg);
            if is_back {
                state.page = Page::Home;
            }
            if is_channel_selected {
                state.page = Page::Player;
            }
            result.map(Message::Player)
        },
        Message::PrepareHome => {
            dbg!("Preparing home, loading:{}", state.home.loading);

            let facade = state.facade.clone();
            Task::perform(
                async move {
                    facade.refresh();
                    // let _ = tokio::task::spawn_blocking(move || ).await;
                },
                |_| Message::HomeReady,
            )
        },
        Message::HomeReady => {
            dbg!("Home ready");
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
        Page::Player => player::view(&state.player).map(Message::Player),
    }
}

/// Returns the active theme.
pub fn theme(_state: &App) -> Theme {
    Theme::Dark
}