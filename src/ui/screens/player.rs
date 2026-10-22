use std::sync::Arc;
use std::time::Duration;
use iced::widget::{button, column, container, row, slider, text};
use iced::{Element, Fill, Padding, Task, alignment};
use iced_aw::widgets::spinner::Spinner;
use iced_video_player::{Video, VideoPlayer};
use libcore::domain::Stream;
use libcore::facade::{CoreFacade, IptvFacade};
use log::{info, warn};
use tokio::sync::mpsc::UnboundedSender;
use crate::player::commands::PlayerEvent;
use crate::ui::components::icon::{ icon_button, SKIP_BACK, PAUSE, PLAY, SKIP_FORWARD, VOLUME_OFF, VOLUME_ON, FULLSCREEN_EXIT, FULLSCREEN_ENTER};
use crate::ui::theme;

/// Number of idle (periodic) ticks that may pass before the control panel is
/// auto-hidden. The periodic ticker fires roughly every 500&nbsp;ms.
const AUTO_HIDE_TICKS: u32 = 6;

/// How many seconds a "skip" step moves the playback position.
const SKIP_SECS: i64 = 10;

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
    /// Nudge the position by a relative delta (seconds). +/- for forward/back.
    SeekDelta(i64),
    /// Volume ratio in the range 0.0..=1.0.
    VolumeChanged(f32),
    ToggleMute,
    ToggleFullscreen,
    /// Periodic ticker message; refreshes position and drives auto-hide.
    Tick,
    /// Emitted when the user moves the mouse over the video area.
    ShowControls,
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
#[derive(Debug)]
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
    /// Most recently observed playback position (seconds).
    pub position_secs: u64,
    /// Duration of the current stream (seconds). 0 while unknown / live.
    pub duration_secs: u64,
    /// Volume in the range 0.0..=1.0.
    pub volume: f32,
    pub is_muted: bool,
    pub is_fullscreen: bool,
    /// Whether the on-video control bar is currently shown.
    pub controls_visible: bool,
    /// Count of idle ticker messages since the last interaction.
    idle_ticks: u32,
    // This channel sends player events to libcore about player states.
    pub event_tx: Option<UnboundedSender<PlayerEvent>>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            loaded_video: None,
            is_playing: false,
            is_loading: false,
            error_message: String::new(),
            channel_name: String::new(),
            channel_id: String::new(),
            is_back_requested: false,
            position_secs: 0,
            duration_secs: 0,
            volume: 1.0,
            is_muted: false,
            is_fullscreen: false,
            controls_visible: true,
            idle_ticks: 0,
            event_tx: None,
        }
    }
}

impl PlayerState {
    /// Formats a number of whole seconds as `m:ss`.
    fn format_time(secs: u64) -> String {
        let minutes = secs / 60;
        let seconds = secs % 60;
        let seconds_str = if seconds < 10 {
            format!("0{}", seconds)
        } else {
            format!("{}", seconds)
        };
        format!("{}:{}", minutes, seconds_str)
    }

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
                state.position_secs = position as u64;
            }
            Task::none()
        }
        PlayerMessage::SeekDelta(delta_secs) => {
            state.idle_ticks = 0;
            state.controls_visible = true;
            let target = (state.position_secs as i64) + delta_secs;
            Task::done(PlayerMessage::ApplySeek(
                if target > 0 { target as u32 } else { 0 },
            ))
        }
        PlayerMessage::VolumeChanged(volume) => {
            info!("[player] VolumeChanged({})", volume);
            state.idle_ticks = 0;
            state.controls_visible = true;
            state.volume = volume;
            if let Some(video) = state.loaded_video.as_mut() {
                video.set_volume(volume as f64);
            }
            Task::none()
        }
        PlayerMessage::ToggleMute => {
            state.idle_ticks = 0;
            state.controls_visible = true;
            state.is_muted = !state.is_muted;
            if let Some(video) = state.loaded_video.as_mut() {
                video.set_muted(state.is_muted);
            }
            Task::none()
        }
        PlayerMessage::ToggleFullscreen => {
            state.idle_ticks = 0;
            state.controls_visible = true;
            state.is_fullscreen = !state.is_fullscreen;
            let mode = if state.is_fullscreen {
                iced::window::Mode::Fullscreen
            } else {
                iced::window::Mode::Windowed
            };
            iced::window::latest()
                .and_then(move |id| iced::window::set_mode::<PlayerMessage>(id, mode))
        }
        PlayerMessage::Tick => {
            if let Some(video) = state.loaded_video.as_ref() {
                state.position_secs = video.position().as_secs();
                state.duration_secs = video.duration().as_secs();
            }
            // Auto-hide only applies once playback is actually running; while
            // idle (nothing loaded / paused) the controls must stay visible so
            // the user can still press play.
            if state.is_playing && state.controls_visible {
                state.idle_ticks += 1;
                if state.idle_ticks >= AUTO_HIDE_TICKS {
                    state.controls_visible = false;
                }
            }
            Task::none()
        }
        PlayerMessage::ShowControls => {
            state.idle_ticks = 0;
            state.controls_visible = true;
            Task::none()
        }
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
    // Base layer of the stack: the video widget always fills the whole area.
    // No placeholder text is shown when there is nothing to play yet.
    let video_widget: Element<'a, PlayerMessage> = if let Some(vid) = video {
        VideoPlayer::new(vid).width(Fill).height(Fill).into()
    } else {
        iced::widget::Space::new().width(Fill).height(Fill).into()
    };

    // ── Timeline (seek slider, or passive bar for live) ────────────────
    let timeline: Element<'a, PlayerMessage> =
        iced::widget::progress_bar(0.0..=100.0, 100.0)
            .length(Fill)
            .girth(iced::Length::Fixed(4.0))
            .into();

    let time_labels = row![
        text(PlayerState::format_time(state.position_secs))
            .size(theme::FONT_SIZE_SM)
            .color(theme::TEXT_PRIMARY),
        text(format!(" / {}", PlayerState::format_time(state.duration_secs)))
            .size(theme::FONT_SIZE_SM)
            .color(theme::TEXT_SECONDARY),
    ]
    .spacing(theme::SPACING_XS)
    .align_y(alignment::Vertical::Center);

    // ... existing code ...
    // Transport buttons (skip back, play/pause, skip forward). Each one is an
    // SVG in a fixed box, so their optical centers coincide exactly.
    let skip_back: Element<'a, PlayerMessage> =
        icon_button(&SKIP_BACK, transport_style, PlayerMessage::SeekDelta(-SKIP_SECS));

    let play_pause: Element<'a, PlayerMessage> = icon_button(
        if state.is_playing { &PAUSE } else { &PLAY },
        |_theme: &iced::Theme, _status| {
            use iced::widget::button::Style;
            Style {
                background: Some(iced::Background::Color(if state.is_playing {
                    theme::PRIMARY
                } else {
                    theme::ACCENT
                })),
                border: iced::Border {
                    radius: theme::CONTROLS_RADIUS.into(),
                    ..Default::default()
                },
                ..Style::default()
            }
        },
        PlayerMessage::PlayPauseToggled(!state.is_playing),
    );

    let skip_forward: Element<'a, PlayerMessage> =
        icon_button(&SKIP_FORWARD, transport_style, PlayerMessage::SeekDelta(SKIP_SECS));

    // Volume group: compact slider + mute toggle.
    let volume_slider = slider(
        0.0..=100.0,
        (state.volume as f64) * 100.0,
        |value: f64| PlayerMessage::VolumeChanged((value / 100.0) as f32),
    )
        .height(4.0)
        .width(120.0);

    let mute_icon: Element<'a, PlayerMessage> = icon_button(
        if state.is_muted { &VOLUME_OFF } else { &VOLUME_ON },
        transport_style,
        PlayerMessage::ToggleMute,
    );

    let fullscreen_icon: Element<'a, PlayerMessage> = icon_button(
        if state.is_fullscreen {
            &FULLSCREEN_EXIT
        } else {
            &FULLSCREEN_ENTER
        },
        transport_style,
        PlayerMessage::ToggleFullscreen,
    );

    // Transport + volume row. All icon buttons are identical 36x36 boxes with an
    // exactly centered 20px glyph, so `align_y(Center)` aligns the glyphs too.
    let controls_inner: Element<'a, PlayerMessage> = container(
        column![
            container(time_labels)
                .width(Fill)
                .align_x(alignment::Horizontal::Right),
            row![
                skip_back,
                play_pause,
                skip_forward,
                iced::widget::Space::new().width(Fill).height(Fill),
                volume_slider,
                mute_icon,
                fullscreen_icon,
            ]
            .spacing(theme::SPACING_MD)
            .align_y(alignment::Vertical::Center),
        ]
            .spacing(theme::SPACING_XS),
    )
        .width(Fill)
        .padding([theme::SPACING_MD, theme::SPACING_LG])
        .into();

    // The visible bar (background) hugging the bottom once bottom-aligned.
    let control_bar: Element<'a, PlayerMessage> = container(
        column![
            timeline,
            controls_inner,
        ]
        .spacing(theme::SPACING_SM),
    )
    .width(Fill)
    .height(90)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center)
    .padding([theme::SPACING_XS, theme::SPACING_LG])
    .into();

    // ── Control panel: transparent bottom overlay ──────────────────────
    let control_overlay: Element<'a, PlayerMessage> = container(
        control_bar,
    )
    .width(Fill)
    .height(Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Bottom)
    .into();

    // Center overlays for loading state / errors.
    let loading_overlay: Element<'a, PlayerMessage> = container(
        Spinner::default().width(60).height(60),
    )
    .center_x(Fill)
    .center_y(Fill)
    .into();

    let error_overlay: Element<'a, PlayerMessage> = container(
        text(&state.error_message).size(theme::FONT_SIZE_LG).color(theme::ACCENT),
    )
    .center_x(Fill)
    .center_y(Fill)
    .into();

    // ── Video area: video is the base layer, controls overlay on top ────
    let mut layers: Vec<Element<'a, PlayerMessage>> = vec![video_widget];

    // Controls draw on top of the video; video size is untouched.
    if state.controls_visible
        && !state.is_loading
        && state.error_message.is_empty()
    {
        layers.push(control_overlay);
    }

    if state.is_loading {
        layers.push(loading_overlay);
    } else if !state.error_message.is_empty() {
        layers.push(error_overlay);
    }

    let video_layer = iced::widget::stack(layers)
        .width(Fill)
        .height(Fill)
        .clip(true);

    // Wrapping the whole thing in a mouse area lets moving the cursor over the
    // video re-show the control bar after it auto-hides.
    let mouse_layer: Element<'a, PlayerMessage> = iced::widget::mouse_area(video_layer)
        .on_move(|_point| PlayerMessage::ShowControls)
        .on_enter(PlayerMessage::ShowControls)
        .into();

    let video_area = container(
        mouse_layer,
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


/// Neutral styling shared by the transport / utility icon buttons.
fn transport_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    use iced::widget::button::{Status, Style};
    let background = match status {
        Status::Hovered => theme::SURFACE_HOVER,
        Status::Pressed => theme::PRIMARY_HOVER,
        _ => theme::SURFACE,
    };
    Style {
        background: Some(iced::Background::Color(background)),
        text_color: theme::TEXT_PRIMARY,
        border: iced::Border {
            radius: theme::RADIUS_SM.into(),
            ..Default::default()
        },
        ..Style::default()
    }
}