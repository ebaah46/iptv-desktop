use iced::widget::{column, container, text};
use iced::{Element, Fill, Task};

use crate::ui::theme;

/// Messages handled by the player screen.
#[derive(Debug, Clone)]
pub enum PlayerMessage {
    ChannelSelected(String),
    PlayPauseToggled(bool),
    BackRequested,
}

/// State consumed by the player screen.
#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub is_playing: bool,
    pub is_loading: bool,
    pub error_message: String,
    pub channel_name: String,
}

/// Processes a player screen message and mutates state.
pub fn update(state: &mut PlayerState, message: PlayerMessage) -> Task<PlayerMessage> {
    match message {
        PlayerMessage::ChannelSelected(id) => {
            state.channel_name = id;
            state.is_playing = true;
        }
        PlayerMessage::PlayPauseToggled(playing) => {
            state.is_playing = playing;
        }
        PlayerMessage::BackRequested => {
            *state = PlayerState::default();
        }
    }
    Task::none()
}

/// Renders the player screen content.
pub fn view(state: &PlayerState) -> Element<'_, PlayerMessage> {
    container(
        column![
            text("Player Screen")
                .size(theme::FONT_SIZE_XL)
                .color(theme::TEXT_PRIMARY),
            text("Video player will render here")
                .size(theme::FONT_SIZE_MD)
                .color(theme::TEXT_SECONDARY),
        ]
        .spacing(theme::SPACING_MD)
        .padding(theme::SPACING_XL),
    )
    .width(Fill)
    .height(Fill)
    .into()
}