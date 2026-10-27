pub mod app;
pub mod components;
pub mod message;
pub mod page;
pub mod screens;
pub mod theme;

use std::sync::{Arc, Mutex};
use iced::Task;

use libcore::facade::IptvFacade;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::player::commands::{PlayerCommand, PlayerEvent};
use crate::ui::message::Message::PrepareHome;
use self::app::{update, view, App, theme};
use self::message::Message;
use self::page::{WINDOW_SIZE, WINDOW_TITLE};

/// Spawns the Iced application window with the given facade and player controller.
pub fn run(facade: Arc<IptvFacade>,  command_rx: UnboundedReceiver<PlayerCommand>, event_tx: UnboundedSender<PlayerEvent>) -> iced::Result {
    let init = Mutex::new(Some((command_rx, event_tx)));

    let boot = move || {
        let (command_rx, event_tx) = init
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
            .expect("boot function execution called 2 or more times");
        (
            App::new(facade.clone(), command_rx, event_tx),
            Task::<Message>::done(PrepareHome),
        )
    };
    iced::application(boot, update, view)
        .theme(theme)
        .title(WINDOW_TITLE)
        .window_size(WINDOW_SIZE)
        .subscription(move |state| state.subscription())
        .run()
}

