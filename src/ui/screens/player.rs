use iced::widget::{column, container, text};
use iced::{Element, Fill};

use crate::ui::theme;

/// View-model data consumed by the player screen.
#[derive(Debug, Clone, Default)]
pub struct PlayerViewModel {
    pub is_playing: bool,
    pub is_loading: bool,
    pub error_message: String,
    pub channel_name: String,
}

/// Renders the player screen content.
pub fn view<Message: 'static + Clone>(
    _model: &PlayerViewModel,
) -> Element<'_, Message> {
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