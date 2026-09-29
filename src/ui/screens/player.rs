use std::sync::{Arc, RwLock};

use iced::widget::{button, column, container, text};
use iced::{Center, Element, Fill, Padding, Task};
use iced_video_player::{Video, VideoPlayer};
use libcore::facade::{CoreFacade, IptvFacade};
use log::info;

use crate::ui::theme;

/// Messages handled by the player screen.
#[derive(Debug, Clone)]
pub enum PlayerMessage {
    PlayPauseToggled(bool),
    BackRequested,
    /// Sent after facade.play() finishes — Video is in the shared lock.
    VideoLoaded,
    /// Sent after facade.pause() finishes.
    Paused,
    /// Sent after facade.stop() finishes.
    Stopped,
}

/// State consumed by the player screen.
#[derive(Debug, Default)]
pub struct PlayerState {
    /// Shared lock containing the currently loaded Video (same Arc as GstPlayerController).
    pub video: Arc<RwLock<Option<Video>>>,
    /// Owned Video taken from the shared lock after loading completes.
    pub loaded_video: Option<Video>,
    pub is_playing: bool,
    pub is_loading: bool,
    pub error_message: String,
    pub channel_name: String,
    pub channel_id: String,
    /// Set by update() to signal app.rs to navigate back.
    pub is_back_requested: bool,
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
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();
            std::thread::spawn(move || {
                info!("[player] Background: calling facade.play({})", &id);
                facade.play(&id);
                info!("[player] Background: facade.play() returned");
                let _ = tx.send(());
            });
            Task::perform(
                async move {
                    let _ = rx.await;
                    PlayerMessage::VideoLoaded
                },
                std::convert::identity,
            )
        }
        PlayerMessage::VideoLoaded => {
            info!("[player] VideoLoaded — taking Video from shared lock");
            state.is_loading = false;
            if let Ok(mut guard) = state.video.write() {
                state.loaded_video = guard.take();
                if state.loaded_video.is_some() {
                    state.is_playing = true;
                    info!("[player] Video stored successfully");
                } else {
                    state.error_message = "Failed to load video".to_string();
                    info!("[player] Video was None — playback failed");
                }
            } else {
                state.error_message = "Failed to load video".to_string();
                info!("[player] Could not acquire write lock");
            }
            Task::none()
        }
        PlayerMessage::PlayPauseToggled(false) => {
            info!("[player] PlayPauseToggled(false) — pausing");
            state.is_playing = false;
            Task::perform(
                tokio::task::spawn_blocking(move || {
                    info!("[player] Background: calling facade.pause()");
                    facade.pause();
                }),
                |_| PlayerMessage::Paused,
            )
        }
        PlayerMessage::Paused => {
            info!("[player] Paused");
            Task::none()
        }
        PlayerMessage::BackRequested => {
            info!("[player] BackRequested — stopping playback");
            Task::perform(
                tokio::task::spawn_blocking(move || {
                    info!("[player] Background: calling facade.stop()");
                    facade.stop();
                }),
                |_| PlayerMessage::Stopped,
            )
        }
        PlayerMessage::Stopped => {
            info!("[player] Stopped — resetting state");
            *state = PlayerState::default();
            state.is_back_requested = true;
            Task::none()
        }
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