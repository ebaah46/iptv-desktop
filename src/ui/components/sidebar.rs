use iced::widget::{button, column, container, scrollable, text};
use iced::{Element, Fill};

use crate::ui::theme;

/// State tracked by the sidebar.
#[derive(Debug, Clone)]
pub struct SidebarState {
    /// All category items.
    pub items: Vec<String>,
    /// Index of the currently selected item.
    pub active_index: usize,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            items: vec![
                "Favorites".into(),
                "Recent".into(),
                "All".into(),
                "Sports".into(),
                "News".into(),
                "Movies".into(),
                "Documentaries".into(),
                "Music".into(),
                "Kids".into(),
            ],
            active_index: 2,
        }
    }
}

/// Renders the sidebar.
pub fn view<Message: 'static + Clone>(
    items: Vec<String>,
    active_index: usize,
    on_selected: impl Fn(usize) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    let title = text("Categories")
        .size(21.0)
        .width(Fill)
        .center()
        .color(theme::TEXT_PRIMARY);

    let rendered_items: Vec<Element<'static, Message>> = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let is_active = i == active_index;

            let label = text(item)
                .size(theme::FONT_SIZE_MD)
                .color(if is_active {
                    theme::TEXT_PRIMARY
                } else {
                    theme::TEXT_SECONDARY
                })
                .font(if is_active {
                    iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..iced::Font::DEFAULT
                    }
                } else {
                    iced::Font::DEFAULT
                });

            let btn = button(label)
                .on_press(on_selected(i))
                .width(Fill)
                .style(move |_theme: &iced::Theme, status| {
                    use iced::widget::button::{Status, Style};

                    let base = if is_active {
                        theme::SIDEBAR_BG
                    } else {
                        iced::Color::TRANSPARENT
                    };

                    let bg = match status {
                        Status::Active => base,
                        Status::Hovered => theme::SURFACE_HOVER,
                        Status::Pressed => theme::SURFACE,
                        Status::Disabled => base,
                    };

                    Style {
                        background: Some(iced::Background::Color(bg)),
                        text_color: if is_active {
                            theme::TEXT_PRIMARY
                        } else {
                            theme::TEXT_SECONDARY
                        },
                        ..Style::default()
                    }
                });

            container(btn)
                .width(Fill)
                .height(36.0)
                .into()
        })
        .collect();

    let list = column(rendered_items)
        .spacing(4.0)
        .padding([0.0, theme::SPACING_SM]);

    let content = column![title, scrollable(list)]
        .spacing(theme::SPACING_MD)
        .padding([theme::SPACING_LG, 0.0]);

    container(content)
        .width(theme::SIDEBAR_WIDTH)
        .height(Fill)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::SIDEBAR_BG)),
                ..Style::default()
            }
        })
        .into()
}