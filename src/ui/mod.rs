pub mod app;
pub mod components;
pub mod message;
pub mod page;
pub mod screens;
pub mod theme;

use std::sync::Arc;

use iced::{Task};

use libcore::facade::IptvFacade;
use crate::ui::message::Message::PrepareHome;
use crate::ui::screens::home::HomeMessage;
use self::app::{update, view, App, theme};
use self::message::Message;
use self::page::{WINDOW_SIZE, WINDOW_TITLE};

/// Spawns the Iced application window with the given facade.
pub fn run(facade: Arc<IptvFacade>) -> iced::Result {
    let boot = move || (App::new(facade.clone()), Task::<Message>::done(PrepareHome));

    iced::application(boot, update, view)
        .theme(theme)
        .title(WINDOW_TITLE)
        .window_size(WINDOW_SIZE)
        .run()
}