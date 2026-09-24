use iced::widget::{container, row, scrollable, text};
use iced::{Element, Fill};

use libcore::domain::Category;

use crate::ui::theme;

/// Renders a horizontal row of category filter tabs.
pub fn view<Message: 'static + Clone>(
    categories: &[Category],
    top_count: usize,
    active_category: &str,
    on_selected: impl Fn(String) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    let all_tab = tab("All".into(), active_category == "All", on_selected("All".into()));

    let category_tabs: Vec<Element<'static, Message>> = categories
        .iter()
        .take(top_count)
        .map(|cat| {
            let is_active = cat.id == active_category;
            tab(cat.name.clone(), is_active, on_selected(cat.id.clone()))
        })
        .collect();

    let mut tabs = vec![all_tab];
    tabs.extend(category_tabs);

    scrollable(
        row(tabs)
            .spacing(theme::SPACING_SM)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(Fill)
    .height(40)
    .into()
}

fn tab<Message: 'static + Clone>(
    label: String,
    is_active: bool,
    on_press: Message,
) -> Element<'static, Message> {
    let label = text(label)
        .size(theme::FONT_SIZE_SM)
        .color(if is_active {
            iced::Color::WHITE
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

    let btn = iced::widget::button(label)
        .on_press(on_press)
        .width(100)
        .height(32)
        .style(move |_theme: &iced::Theme, _status| {
            use iced::widget::button::Style;
            Style {
                background: if is_active {
                    Some(iced::Background::Color(theme::ACCENT))
                } else {
                    Some(iced::Background::Color(theme::SURFACE))
                },
                text_color: if is_active {
                    iced::Color::WHITE
                } else {
                    theme::TEXT_SECONDARY
                },
                border: iced::Border {
                    width: 1.0,
                    color: if is_active { theme::ACCENT } else { theme::BORDER_COLOR },
                    radius: theme::RADIUS_MD.into(),
                },
                ..Style::default()
            }
        });

    container(btn).into()
}