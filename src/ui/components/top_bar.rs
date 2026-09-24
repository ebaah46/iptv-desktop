use iced::widget::{button, container, row, text, text_input, Space};
use iced::{Element, Fill, Font};

use crate::ui::theme;

/// Default navigation tabs shown in the top bar.
pub const TABS: &[&str] = &["Home", "Guide", "Movies", "Series", "Recordings"];

/// Renders the top bar.
pub fn view<Message: 'static + Clone>(
    active_tab: &str,
    search_query: &str,
    on_tab_selected: impl Fn(String) -> Message + 'static + Clone,
    on_search_changed: impl Fn(String) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    // ── Logo area ──────────────────────────────────────────────────────
    let logo = row![
        container(text(" ").size(12).color(theme::TEXT_PRIMARY))
            .width(28.0)
            .height(28.0)
            .style(|_theme: &iced::Theme| {
                use iced::widget::container::Style;
                Style {
                    background: Some(iced::Background::Color(theme::ACCENT)),
                    ..Style::default()
                }
            }),
        text("IPTV")
            .size(theme::FONT_SIZE_LG)
            .color(theme::TEXT_PRIMARY)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Font::DEFAULT
            }),
    ]
    .spacing(theme::SPACING_SM)
    .align_y(iced::alignment::Vertical::Center);

    // ── Navigation tabs ────────────────────────────────────────────────
    let tabs: Vec<Element<'static, Message>> = TABS
        .iter()
        .map(|tab| {
            let is_active = *tab == active_tab;
            let label = text(*tab)
                .size(theme::FONT_SIZE_MD)
                .color(if is_active {
                    iced::Color::WHITE
                } else {
                    theme::TEXT_SECONDARY
                })
                .font(Font {
                    weight: if is_active {
                        iced::font::Weight::Semibold
                    } else {
                        iced::font::Weight::Normal
                    },
                    ..Font::DEFAULT
                });

            let btn = button(label)
                .on_press(on_tab_selected(tab.to_string()))
                .height(36)
                .style(move |_theme: &iced::Theme, _status| {
                    use iced::widget::button::Style;
                    Style {
                        background: if is_active {
                            Some(iced::Background::Color(theme::PRIMARY))
                        } else {
                            None
                        },
                        text_color: if is_active {
                            iced::Color::WHITE
                        } else {
                            theme::TEXT_SECONDARY
                        },
                        border: iced::Border {
                            radius: theme::RADIUS_SM.into(),
                            ..iced::Border::default()
                        },
                        ..Style::default()
                    }
                });

            container(btn).into()
        })
        .collect();

    let tabs_row = row(tabs)
        .spacing(theme::SPACING_XS)
        .align_y(iced::alignment::Vertical::Center)
        .width(Fill);

    // ── Search + settings ──────────────────────────────────────────────
    let search_input = text_input("Enter channel search here...", search_query)
        .on_input(on_search_changed)
        .size(theme::FONT_SIZE_MD)
        .padding([0.0, theme::SPACING_SM]);

    let search_bar = container(
        row![
            text("🔍").size(theme::FONT_SIZE_MD),
            search_input.width(Fill),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::alignment::Vertical::Center),
    )
    .width(250)
    .height(32);

    let settings_icon = container(text("⚙️").size(theme::FONT_SIZE_MD))
        .width(32)
        .height(32)
        .center_x(Fill)
        .center_y(Fill);

    let right_section = row![search_bar, settings_icon]
        .spacing(theme::SPACING_SM)
        .align_y(iced::alignment::Vertical::Center);

    // ── Assemble top bar ───────────────────────────────────────────────
    let bar = row![
        container(logo)
            .padding([0.0, theme::SPACING_LG])
            .center_y(Fill),
        tabs_row,
        Space::new().width(Fill).height(Fill),
        right_section,
    ]
    .spacing(0)
    .align_y(iced::alignment::Vertical::Center)
    .height(theme::TOPBAR_HEIGHT);

    container(bar)
        .width(Fill)
        .height(theme::TOPBAR_HEIGHT)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::TOPBAR_BG)),
                ..Style::default()
            }
        })
        .into()
}