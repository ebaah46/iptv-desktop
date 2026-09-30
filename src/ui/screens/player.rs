use std::sync::Arc;
use std::time::Duration;
use iced::widget::{button, column, container, text};
use iced::{Center, Element, Fill, Padding, Task};
use iced_video_player::{Error, Video, VideoPlayer};
use libcore::domain::Stream;
use libcore::facade::{CoreFacade, IptvFacade};
use log::{info, warn};
use tokio::sync::mpsc::UnboundedSender;
use crate::player::commands::PlayerEvent;
use crate::ui::theme;

/// Messages handled by the player screen.
#[derive(Debug, Clone)]
pub enum PlayerMessage {
    PlayPauseToggled(bool),
    BackRequested,
    ApplyLoad(Stream),
    ApplyPlay,
    ApplyPause,
    ApplyStop,
    ApplySeek(u32),
    /// Sent after video is loaded into the UI widget.
    VideoLoaded,
    /// Sent after video starts playing
    Playing,
    /// Sent after facade.pause() finishes.
    Paused,
    /// Sent after facade.stop() finishes.
    Stopped,
}

/// State consumed by the player screen.
#[derive(Debug, Default)]
pub struct PlayerState {
    /// Owned Video taken from the application level.
    pub loaded_video: Option<Video>,
    pub is_playing: bool,
    pub is_loading: bool,
    pub error_message: String,
    pub channel_name: String,
    pub channel_id: String,
    /// Set by update() to signal app.rs to navigate back.
    pub is_back_requested: bool,
    // This channel sends player events to libcore about player states.
    pub event_tx: Option<UnboundedSender<PlayerEvent>>,
}

/// Processes a player screen message and mutates state.
pub fn update(
    state: &mut PlayerState,
    message: PlayerMessage,
    facade: Arc<IptvFacade>,
) -> Task<PlayerMessage> {
    match message {
        PlayerMessage::PlayPauseToggled(true) => {
            info!("[player] PlayPauseToggled(true) — starting playback");
            state.is_loading = true;
            let id = state.channel_id.clone();
            let facade = facade.clone();
            tokio::task::spawn_blocking(move || {
                facade.play(&id);
            });
            Task::none()
        }
        PlayerMessage::VideoLoaded => {
            info!("[player] VideoLoaded — playback to start shortly.");
            state.is_loading = false;
            state.is_playing = true;
            Task::none()
        }
        PlayerMessage::PlayPauseToggled(false) => {
            info!("[player] PlayPauseToggled(false) — pausing");
            state.is_playing = false;
            facade.pause();
            Task::none()
        }
        PlayerMessage::Paused => {
            info!("[player] Paused");
            Task::none()
        }
        PlayerMessage::BackRequested => {
            info!("[player] BackRequested — stopping playback");
            Task::perform(
                async move {
                    info!("[player] calling facade.stop()");
                    facade.stop();
                },
                |_| PlayerMessage::Stopped,
            )
        }
        PlayerMessage::Stopped => {
            info!("[player] Stopped — resetting state");
            *state = PlayerState::default();
            state.is_back_requested = true;
            Task::none()
        }
        PlayerMessage::ApplyLoad(stream) => {
            let result = url::Url::parse(&stream.url)
                .ok()
                .and_then(|url| Video::new(&url).ok());

            state.loaded_video = result;

            let event = match state.loaded_video {
                Some(_) => PlayerEvent::Started,
                None => PlayerEvent::Failed("load failed".into()),
            };
            // Send event back to libcore about the load operation
            state.event_tx.as_ref().map(|tx| tx.send(event));
            Task::done(PlayerMessage::VideoLoaded)
        },

        PlayerMessage::ApplyPlay => {
            if let Some(video) = state.loaded_video.as_mut(){
                video.set_paused(false)
            } else {
                warn!("[player] Could not set playback state, loaded video not available");
            }
            Task::none()
        },
        PlayerMessage::ApplyPause => {
            if let Some(video) = state.loaded_video.as_mut() {
                video.set_paused(true)
            } else {
                warn!("[player] Could not set playback state");
            }
            Task::done(PlayerMessage::Paused)
        },
        PlayerMessage::ApplyStop => {
            if let Some(video) = state.loaded_video.as_mut() {
                video.set_paused(true);
                let event  = match video.seek(Duration::ZERO, false){
                    Ok(_) => PlayerEvent::Stopped,
                    Err(_) => PlayerEvent::Failed("Failed to complete video stop process".into()),
                };
                state.event_tx.as_ref().map(|tx| tx.send(event));
            }
            Task::done(PlayerMessage::Stopped)
        },
        PlayerMessage::ApplySeek(position) => {
            if let Some(video) = state.loaded_video.as_mut() {
                let target_time = Duration::from_secs(position as u64);
                let current_time = video.position();
                if target_time > current_time {
                    info!("Seeking to future time not possible. Current time: {}, Target time: {}",current_time.as_secs(), target_time.as_secs());
                    return Task::none();
                }
                let _ = video.seek(target_time, false);
            }
            Task::none()
        },
        PlayerMessage::Playing => Task::none(),
    }
}

/// Renders the player screen content.
///
/// `video` is the currently loaded Video (from `PlayerState.loaded_video`).
pub fn view<'a>(
    state: &'a PlayerState,
    video: Option<&'a Video>,
) -> Element<'a, PlayerMessage> {
    // ── Back button ─────────────────────────────────────────────────────
    let back_btn = container(
        button(text("← Back").size(theme::FONT_SIZE_MD).color(theme::TEXT_PRIMARY))
            .padding([8, 16])
            .style(|_theme: &iced::Theme, _status| {
                use iced::widget::button::Style;
                Style {
                    background: Some(iced::Background::Color(theme::SURFACE)),
                    border: iced::Border {
                        radius: theme::RADIUS_SM.into(),
                        ..Default::default()
                    },
                    ..Style::default()
                }
            })
            .on_press(PlayerMessage::BackRequested),
    )
    .width(Fill)
    .padding(Padding {
        top: 0.0,
        right: 0.0,
        bottom: theme::SPACING_MD,
        left: 0.0,
    });

    // ── Channel name ────────────────────────────────────────────────────
    let channel_name = text(format!("Now Playing: {}", state.channel_name))
        .size(theme::FONT_SIZE_XL)
        .color(theme::TEXT_PRIMARY)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::DEFAULT
        });

    // ── Video player widget ────────────────────────────────────────────
    let video_widget: Element<'a, PlayerMessage> = if let Some(vid) = video {
        VideoPlayer::new(vid).width(Fill).height(Fill).into()
    } else {
        container(
            text("Select a channel to play")
                .size(theme::FONT_SIZE_LG)
                .color(theme::TEXT_SECONDARY),
        )
        .center_x(Fill)
        .center_y(Fill)
        .width(Fill)
        .height(Fill)
        .into()
    };

    // ── Control overlay (centered on top of video) ──────────────────────
    let overlay: Element<'a, PlayerMessage> = if state.is_loading {
        container(
            text("Loading stream...")
                .size(theme::FONT_SIZE_LG)
                .color(theme::TEXT_SECONDARY),
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    } else if !state.error_message.is_empty() {
        container(
            text(&state.error_message)
                .size(theme::FONT_SIZE_LG)
                .color(theme::ACCENT),
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    } else if state.is_playing {
        // Pause button
        container(
            button(
                text("⏸").size(32).color(theme::TEXT_PRIMARY),
            )
            .padding(20)
            .style(|_theme: &iced::Theme, _status| {
                use iced::widget::button::Style;
                Style {
                    background: Some(iced::Background::Color(theme::PRIMARY)),
                    border: iced::Border {
                        radius: 40.0.into(),
                        ..Default::default()
                    },
                    ..Style::default()
                }
            })
            .on_press(PlayerMessage::PlayPauseToggled(false)),
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    } else {
        // Play button (shown when not playing, not loading, no error)
        container(
            button(
                text("▶").size(32).color(theme::TEXT_PRIMARY),
            )
            .padding(20)
            .style(|_theme: &iced::Theme, _status| {
                use iced::widget::button::Style;
                Style {
                    background: Some(iced::Background::Color(theme::ACCENT)),
                    border: iced::Border {
                        radius: 40.0.into(),
                        ..Default::default()
                    },
                    ..Style::default()
                }
            })
            .on_press(PlayerMessage::PlayPauseToggled(true)),
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    };

    // ── Video area with overlay ─────────────────────────────────────────
    let video_area = container(
        column![
            video_widget,
            overlay,
        ]
        .spacing(0),
    )
    .width(Fill)
    .height(Fill)
    .style(|_theme: &iced::Theme| {
        use iced::widget::container::Style;
        Style {
            background: Some(iced::Background::Color(theme::SURFACE)),
            border: iced::Border {
                radius: theme::RADIUS_MD.into(),
                ..Default::default()
            },
            ..Style::default()
        }
    });

    // ── Full layout ─────────────────────────────────────────────────────
    container(
        column![back_btn, channel_name, video_area]
            .spacing(theme::SPACING_MD)
            .padding(theme::SPACING_XL),
    )
    .width(Fill)
    .height(Fill)
    .style(|_theme: &iced::Theme| {
        use iced::widget::container::Style;
        Style {
            background: Some(iced::Background::Color(theme::BACKGROUND)),
            ..Style::default()
        }
    })
    .into()
}
