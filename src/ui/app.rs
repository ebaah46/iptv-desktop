use futures::StreamExt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use iced::{futures::stream, futures::Stream, Element, Subscription, Task, Theme};

use libcore::domain::Logo;
use libcore::facade::{CoreFacade, IptvFacade};
use log::info;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::logo::{LogoLoader, merge_logo_batch, LogoPipeline};
use crate::player::commands::{PlayerCommand, PlayerEvent};
use crate::ui::message::Message;
use crate::ui::page::Page;
use crate::ui::screens::countries;
use crate::ui::screens::countries::CountryState;
use crate::ui::screens::home::{self, HomeState};
use crate::ui::screens::player::{self, PlayerState};
use crate::ui::screens::player::PlayerMessage::{ApplyLoad, ApplyPause, ApplyPlay, ApplySeek, ApplyStop, Tick as PlayerTick};
use crate::ui_model::{ChannelItem, ChannelItems, CountryItem, CountryItems, LogoImage};


/// Root application state.
pub struct App {
    pub facade: Arc<IptvFacade>,
    pub page: Page,
    pub home: HomeState,
    pub player: PlayerState,
    pub country: CountryState,
    pub command_rx: Arc<Mutex<Option<UnboundedReceiver<PlayerCommand>>>>,
    pub logo_pipeline: Arc<LogoPipeline>
}

impl App {
    pub fn new(facade: Arc<IptvFacade>, command_tx: UnboundedReceiver<PlayerCommand>, event_tx: UnboundedSender<PlayerEvent>) -> Self {
        let mut player = PlayerState::default();
        player.event_tx = Some(event_tx.clone());
        let logos = Arc::new(LogoPipeline::new(Arc::new(LogoLoader::new(None))));
        Self {
            facade,
            page: Page::Home,
            home: HomeState::new(),
            player,
            country: CountryState::default(),
            command_rx: Arc::new(Mutex::new(Some(command_tx))),
            logo_pipeline: logos
        }
    }
    /// Called repeatedly by iced. Must return a stable, pure recipe.
    pub fn subscription(&self) -> Subscription<Message> {
        let id = PlayerCommandsId {
            rx_slot: self.command_rx.clone(),
        };
        let commands = Subscription::run_with(id, Self::player_commands_stream);

        // Only the Player screen consumes ticks.
        // For updating the time watched counter
        let ticker = if matches!(self.page, Page::Player) {
            iced::time::every(std::time::Duration::from_millis(500))
                .map(|_| Message::Player(PlayerTick))
        } else {
            Subscription::none()
        };

        let logos = Subscription::run_with(LogoPipelineId(self.logo_pipeline.clone()), Self::logo_stream);

        Subscription::batch([commands, ticker, logos])
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

    fn logo_stream(id: &LogoPipelineId) -> Pin<Box<dyn Stream<Item = Message> + Send>> {
        Box::pin(id.0.clone().batch_stream().map(Message::LogoBatch))
    }
}

/// Processes a message and mutates state.
pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::NavigateTo(route) => {
            state.page = route.page;
            helpers::enter(state, &route)
        }
        Message::Home(msg) => {
            // Extract the route *before* delegating (msg is consumed by home::update).
            let route = helpers::route_from_home(&msg);
            let task = home::update(&mut state.home, msg, state.facade.clone(), state.logo_pipeline.as_ref())
                .map(Message::Home);
            match route {
                Some(route) => Task::batch([task, Task::done(Message::NavigateTo(route))]),
                None => task,
            }
        }

        Message::Player(msg) => {
            // Same shape as Home/Country: no `state.page = ...` anywhere but NavigateTo.
            let route = helpers::route_from_player(&msg);
            let task = player::update(&mut state.player, msg, state.facade.clone())
                .map(Message::Player);
            match route {
                Some(route) => Task::batch([task, Task::done(Message::NavigateTo(route))]),
                None => task,
            }
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
            state.country.countries = state.facade.catalog_service.get_countries()
                .iter()
                .map(|country| Arc::new(CountryItem::new(country.clone())))
                .collect();
            info!("[player] # Channels: {} Countries:{}", state.home.channel_count, state.country.countries.len());
            state.home.loading = false;

            state.logo_pipeline.enqueue_all(&state.home.prefetch_window());

            // let stream = load_all(state.logo_loader.clone(), state.home.channel_data.deref().clone());
            Task::none()
        },
        Message::LogoBatch(batch) => {
            // Attach decoded handles to the master channel list, and to any
            // search/filter snapshots are automatically updated because channel item
            // now contains pointer to logo
            merge_logo_batch(&batch, &state.home.channel_data);
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
        Message::Country(msg) => {
            let route = helpers::route_from_country(&msg);
            let task = countries::update(&mut state.country, msg, state.facade.clone()).map(Message::Country);
            match route {
                Some(route) => Task::batch([task, Task::done(Message::NavigateTo(route))]),
                None => task,
            }
        }
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
        Page::Country => countries::view(&state.country).map(Message::Country)
    }
}

/// Returns the active theme.
pub fn theme(_state: &App) -> Theme {
    Theme::Dark
}

mod helpers {
    use iced::Task;
    use crate::ui::app::App;
    use crate::ui::message::Message;
    use crate::ui::page::{Page, Route};
    use crate::ui::screens::countries;
    use crate::ui::screens::home::HomeMessage;
    use crate::ui::screens::countries::CountryMessage;
    use crate::ui::screens::player::PlayerMessage;

    /// Does this home message leave the screen? If so, with what context?
    pub fn route_from_home(msg: &HomeMessage) -> Option<Route> {
        match msg {
            HomeMessage::CountriesClicked => Some(Route::to(Page::Country)),
            HomeMessage::ChannelClicked(id) => Some(Route::focused(Page::Player, id.clone())),
            _ => None,
        }
    }
    pub fn route_from_country(msg: &CountryMessage) -> Option<Route> {
        match msg {
            CountryMessage::BackRequested => Some(Route::to(Page::Home)),
            _ => None,
        }
    }
    pub fn route_from_player(msg: &PlayerMessage) -> Option<Route> {
        match msg {
            PlayerMessage::Exited => Some(Route::to(Page::Home)),
            _ => None,
        }
    }

    /// Enter-time work for a route, in one place. Any future page that needs a
    /// preselection or a load task hooks in here — never in a delegate arm.
    pub fn enter(state: &mut App, route: &Route) -> Task<Message> {
        match route.page {
            Page::Home => Task::none(),
            Page::Player => {
                if let Some(id) = &route.focus {
                    state.player.channel_id = id.clone();
                    state.player.channel_name = id.clone();
                    state.player.error_message.clear();
                }
                Task::none()
            }
            Page::Country => {
                if let Some(code) = &route.focus {
                    state.country.selected_country = code.clone();
                }
                countries::update(&mut state.country, CountryMessage::Entered, state.facade.clone())
                    .map(Message::Country)
            }
        }
    }

}


#[derive(Clone)]
struct PlayerCommandsId {
    rx_slot: Arc<Mutex<Option<UnboundedReceiver<PlayerCommand>>>>,
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


/// Subscription identity for the logo queue; stable across frames because it
/// hashes the pipeline's address.
#[derive(Clone)]
struct LogoPipelineId(Arc<LogoPipeline>);

impl Hash for LogoPipelineId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for LogoPipelineId {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for LogoPipelineId {}