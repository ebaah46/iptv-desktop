use std::sync::Arc;
use iced::widget::{button, column, container, text};
use iced::{Element, Fill, Font};
use iced::Length::Fixed;

use crate::ui::theme;
use crate::ui_model::ChannelItem;

/// Renders a single channel card.
pub fn view<Message: 'static + Clone>(
    item: Arc<ChannelItem>,
    on_selected: impl Fn(String) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    let channel_name = item.name().to_string();
    let logo_name = channel_name.clone();

    // The logo image is sized to at least 60% of the card's height. A missing
    // or empty logo simply renders as reserved blank space (no fallback).
    let logo_size = theme::CHANNEL_CARD_HEIGHT * 0.6;
    let logo_content: Element<'_, Message> = if let Some(handle) = item.logo_handle().as_ref() {
        iced::widget::image::viewer(handle.clone())
            .width(logo_size)
            .height(logo_size)
            .content_fit(iced::ContentFit::Contain)
            .into()
    } else {
        container(text("")).width(logo_size).height(logo_size).into()
    };
    let logo = container(logo_content)
        .width(logo_size)
        .height(logo_size)
        .center_x(Fill)
        .center_y(Fill);

    let name = text(logo_name)
        .size(theme::FONT_SIZE_LG)
        .color(theme::TEXT_PRIMARY)
        .font(Font {
            weight: iced::font::Weight::Semibold,
            ..Font::DEFAULT
        });

    let program = text("Now Playing…")
        .size(theme::FONT_SIZE_MD)
        .color(theme::TEXT_SECONDARY);


    // TODO: should be controlled by current program time left
    let progress_bar = iced::widget::progress_bar(0.0..=100.0, 80.0).length(Fill).girth(Fixed(4.0));

    let body = column![logo, name, program, progress_bar]
        .spacing(theme::SPACING_SM)
        .padding(theme::SPACING_MD);

    button(
        container(body).width(theme::CHANNEL_CARD_WIDTH).height(theme::CHANNEL_CARD_HEIGHT),
    )
    .on_press(on_selected(item.id().to_string()))
    .padding(0)
    .style(move |_theme: &iced::Theme, status| {
        use iced::widget::button::Style;
        use iced::widget::button::Status;
        let background = match status {
            Status::Hovered => Some(iced::Background::Color(theme::SURFACE_HOVER)),
            Status::Pressed => Some(iced::Background::Color(theme::ACCENT)),
            _ => Some(iced::Background::Color(theme::CARD_BG)),
        };
        Style {
            background,
            border: iced::Border {
                width: 1.5,
                color: theme::BORDER_COLOR,
                radius: theme::RADIUS_LG.into(),
            },
            ..Style::default()
        }
    })
    .into()
}