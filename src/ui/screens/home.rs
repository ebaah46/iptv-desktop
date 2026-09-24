use iced::widget::{column, container, text};
use iced::{Element, Fill};

use crate::ui::theme;

/// View-model data consumed by the home screen.
#[derive(Debug, Clone, Default)]
pub struct HomeViewModel {
    pub channel_count: usize,
}

/// Renders the home screen content.
pub fn view<Message: 'static + Clone>(
    _model: &HomeViewModel,
) -> Element<'_, Message> {
    container(
        column![
            text("Home Screen")
                .size(theme::FONT_SIZE_XL)
                .color(theme::TEXT_PRIMARY),
            text("Channels will appear here")
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