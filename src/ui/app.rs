use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use iced::{futures::stream, futures::Stream, Element, Subscription, Task, Theme};
use iced::widget::image::Handle;

use libcore::domain::Logo;
use libcore::facade::{CoreFacade, IptvFacade};
use log::info;
use reqwest::Method;
use reqwest::blocking::Client as HttpClient;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc;
use crate::player::commands::{PlayerCommand, PlayerEvent};
use crate::ui::message::Message;
use crate::ui::page::Page;
use crate::ui::screens::home::{self, HomeMessage, HomeState};
use crate::ui::screens::player::{self, PlayerState};
use crate::ui::screens::player::PlayerMessage::{ApplyLoad, ApplyPause, ApplyPlay, ApplySeek, ApplyStop};
use crate::ui_model::{ChannelItem, ChannelItems};

/// Downloads and decodes a channel logo at the given URL into an image handle.
fn fetch_logo_handle(url: &str) -> Option<Handle> {
    if url.is_empty() {
        return None;
    }
    let client = HttpClient::new();
    client
        .request(Method::GET, url)
        .send()
        .ok()
        .and_then(|r| r.bytes().ok())
        .map(|bytes| Handle::from_bytes(bytes))
}

/// Rebuilds the channel items, attaching a decoded logo handle to each when the
/// logo could be fetched. Items without a logo are cloned as-is.
fn load_logos(items: &Arc<ChannelItems>) -> ChannelItems {
    items.iter().map(|item| {
        match fetch_logo_handle(item.logo_url()) {
            Some(handle) => Arc::new(item.with_logo_handle(handle)),
            None => item.clone(),
        }
    }).collect()
}

/// Root application state.
pub struct App {
    pub facade: Arc<IptvFacade>,
    pub page: Page,
    pub home: HomeState,
    pub player: PlayerState,
    pub command_rx: Arc<Mutex<Option<UnboundedReceiver<PlayerCommand>>>>,
}

impl App {
    pub fn new(facade: Arc<IptvFacade>, command_tx: UnboundedReceiver<PlayerCommand>, event_tx: UnboundedSender<PlayerEvent>) -> Self {
        let mut player = PlayerState::default();
        player.event_tx = Some(event_tx.clone());
        Self {
            facade,
            page: Page::Home,
            home: HomeState::new(),
            player,
            command_rx: Arc::new(Mutex::new(Some(command_tx))),
        }
    }
    /// Called repeatedly by iced. Must return a stable, pure recipe.
    pub fn subscription(&self) -> Subscription<Message> {
        let id = PlayerCommandsId {
            rx_slot: self.command_rx.clone(),
        };

        Subscription::run_with(id, Self::player_commands_stream)
    }

    fn player_commands_stream(
        id: &PlayerCommandsId,
    ) -> Pin<Box<dyn Stream<Item = Message> + Send>>  {
        let rx = {
            let mut guard = id.rx_slot
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.take()
        };

        let rx = match rx {
            Some(rx) => rx,
            None => return Box::pin(stream::empty()),
        };

        Box::pin(stream::unfold(rx, |mut rx| async move {
            rx.recv()
                .await
                .map(|command| (Message::PlayerCommand(command), rx))
        }))
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
            let channels = state.facade.catalog_service.get_active_channels();
            let logos = state.facade.catalog_service.get_logos();

            state.home.channel_count = channels.len();

            // Index logos by channel id for O(1) lookup when pairing items.
            let mut logo_index: HashMap<String, Arc<Logo>> = HashMap::new();
            for l in logos.iter() {
                logo_index.insert(l.channel.clone().into(), l.clone());
            }

            let mut items: ChannelItems = Vec::new();
            for ch in channels.iter() {
                if let Some(logo) = logo_index.get(&ch.id) {
                    items.push(Arc::new(ChannelItem::new(ch.clone(), logo.clone())));
                }
            }

            state.home.channel_data = Arc::new(items);
            state.home.category_data =  state.facade.catalog_service.get_categories();
            state.home.countries_data = state.facade.catalog_service.get_countries();
            info!("[player] # Channels: {}", state.home.channel_count);
            state.home.loading = false;

            // Fetch and decode logos in the background; re-render when ready.
            let items_for_logos = state.home.channel_data.clone();
            Task::perform(
                tokio::task::spawn_blocking(move || {
                    Message::LogosReady(load_logos(&items_for_logos))
                }),
                |res| res.unwrap_or_else(|_| Message::Unknown),
            )
        },
        Message::LogosReady(items) => {
            state.home.channel_data = Arc::new(items);
            Task::none()
        },

        Message::PlayerCommand(command) => {
            let msg = match command {
                PlayerCommand::Load(stream) => ApplyLoad(stream),
                PlayerCommand::Play => ApplyPlay,
                PlayerCommand::Pause => ApplyPause,
                PlayerCommand::Stop => ApplyStop,
                PlayerCommand::Seek(pos) => ApplySeek(pos),
            };
            player::update(&mut state.player, msg, state.facade.clone()).map(Message::Player)
        }

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

#[derive(Clone)]
struct PlayerCommandsId {
    rx_slot: Arc<Mutex<Option<mpsc::UnboundedReceiver<PlayerCommand>>>>,
}

impl Hash for PlayerCommandsId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.rx_slot).hash(state);
    }
}

impl PartialEq for PlayerCommandsId {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.rx_slot, &other.rx_slot)
    }
}
impl Eq for PlayerCommandsId {}
