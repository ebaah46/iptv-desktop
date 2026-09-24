use iced::widget::{column, container, text};
use iced::{Color, Element, Fill, Font};

use libcore::domain::Channel;

use crate::ui::theme;

/// Deterministic colour derived from a string (channel name).
fn color_from_name(name: &str) -> Color {
    let hash: u32 = name
        .bytes()
        .fold(0u32, |h, b| h.wrapping_mul(31).wrapping_add(b as u32));
    let r = ((hash >> 16) & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = (hash & 0xFF) as u8;
    Color::from_rgb8(r, g, b)
}

/// Renders a single channel card.
pub fn view<Message: 'static + Clone>(
    channel: &Channel,
    _on_selected: impl Fn(String) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    let channel_name = channel.name.clone();
    let logo_color = color_from_name(&channel_name);
    let logo_text = channel_name
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let logo_initial = text(logo_text)
        .size(theme::FONT_SIZE_LG)
        .color(Color::WHITE)
        .font(Font {
            weight: iced::font::Weight::Bold,
            ..Font::DEFAULT
        });

    let logo = container(logo_initial)
        .width(40.0)
        .height(40.0)
        .center_x(Fill)
        .center_y(Fill)
        .style(move |_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(logo_color)),
                ..Style::default()
            }
        });

    let name = text(channel_name)
        .size(theme::FONT_SIZE_MD)
        .color(theme::TEXT_PRIMARY)
        .font(Font {
            weight: iced::font::Weight::Semibold,
            ..Font::DEFAULT
        });

    let program = text("Now Playing…")
        .size(theme::FONT_SIZE_SM)
        .color(theme::TEXT_SECONDARY);

    // Progress fill bar (60% wide) overlaid on a background track
    let fill = container(text(""))
        .width(60.0)
        .height(4.0)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::PROGRESS_FILL)),
                ..Style::default()
            }
        });

    let progress_track = container(fill)
        .width(Fill)
        .height(4.0)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::PROGRESS_BG)),
                ..Style::default()
            }
        });

    let body = column![logo, name, program, progress_track]
        .spacing(theme::SPACING_XS)
        .padding(theme::SPACING_SM);

    container(body)
        .width(theme::CHANNEL_CARD_WIDTH)
        .height(theme::CHANNEL_CARD_HEIGHT)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::CARD_BG)),
                border: iced::Border {
                    width: 1.0,
                    color: theme::BORDER_COLOR,
                    radius: theme::RADIUS_MD.into(),
                },
                ..Style::default()
            }
        })
        .into()
}