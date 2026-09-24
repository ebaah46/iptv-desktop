use std::sync::Arc;

use iced::{Element, Task, Theme};

use libcore::facade::{CoreFacade, IptvFacade};

use crate::ui::message::Message;
use crate::ui::page::Page;
use crate::ui::screens::home::{view as home_view, HomeViewModel};
use crate::ui::screens::player::{view as player_view, PlayerViewModel};

/// Root application state.
pub struct App {
    facade: Arc<IptvFacade>,
    page: Page,
    home: HomeViewModel,
    player: PlayerViewModel,
}

impl App {
    pub fn new(facade: Arc<IptvFacade>) -> Self {
        Self {
            facade,
            page: Page::Home,
            home: HomeViewModel::default(),
            player: PlayerViewModel::default(),
        }
    }
}

/// Processes a message and mutates state.
pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::NavigateTo(page) => {
            state.page = page;
        }
        Message::ChannelSelected(id) => {
            state.player.channel_name = id.clone();
            state.player.is_playing = true;
            state.page = Page::Player;
        }
        Message::CategorySelected(_id) => {}
        Message::PlayPauseToggled(playing) => {
            state.player.is_playing = playing;
            if playing {
                state.facade.play("");
            } else {
                state.facade.pause();
            }
        }
        Message::BackRequested => {
            state.player = PlayerViewModel::default();
            state.facade.stop();
            state.page = Page::Home;
        }
    }
    Task::none()
}

/// Renders the current screen.
pub fn view(state: &App) -> Element<'_, Message> {
    match state.page {
        Page::Home => home_view(&state.home),
        Page::Player => player_view(&state.player),
    }
}

/// Returns the active theme.
pub fn theme(_state: &App) -> Theme {
    Theme::Dark
}