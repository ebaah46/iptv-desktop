use std::sync::Arc;

use iced::widget::{column, container, row, scrollable, text, Scrollable};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::{Center, Element, Fill, Font, Task};
use iced_aw::widgets::spinner::Spinner;
use libcore::domain::{Categories, Channels, Countries};
use libcore::facade::IptvFacade;

use crate::ui::components::category_tabs;
use crate::ui::components::channel_card;
use crate::ui::components::sidebar::{self, SidebarState};
use crate::ui::components::status_bar::{self, StatusBarData};
use crate::ui::components::top_bar;
use crate::ui::theme;

/// Messages handled by the home screen.
#[derive(Debug, Clone)]
pub enum HomeMessage {
    CategorySelected(String),
    TabSelected(String),
    SearchChanged(String),
    ChannelClicked(String),
}

/// State consumed by the home screen.
#[derive(Debug, Clone, Default)]
pub struct HomeState {
    pub channel_count: usize,
    pub sidebar: SidebarState,
    pub top_bar_active_tab: String,
    pub top_bar_search_query: String,
    pub status_bar: StatusBarData,
    pub channel_data: Channels,
    pub category_data: Categories,
    pub countries_data: Countries,
    pub loading: bool,
    pub active_category: String,
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            top_bar_active_tab: "Home".into(),
            active_category: "All".into(),
            loading: true,
            ..Self::default()
        }
    }
}

/// Processes a home screen message and mutates state.
pub fn update(
    state: &mut HomeState,
    message: HomeMessage,
    _facade: Arc<IptvFacade>,
) -> Task<HomeMessage> {
    match message {
        HomeMessage::CategorySelected(name) => {
            if let Some(index) = state.sidebar.items.iter().position(|i| *i == name) {
                state.sidebar.active_index = index;
            }
            state.active_category = name;
        }
        HomeMessage::TabSelected(tab) => {
            state.top_bar_active_tab = tab;
        }
        HomeMessage::SearchChanged(query) => {
            state.top_bar_search_query = query;
        }
        HomeMessage::ChannelClicked(_id) => {
            // Navigation to player will be handled at the app level.
        }
    }
    Task::none()
}

/// Renders the home screen content.
pub fn view(state: &HomeState) -> Element<'_, HomeMessage> {
    let items = state.sidebar.items.clone();
    let active_index = state.sidebar.active_index;

    let sidebar = sidebar::view(items, active_index, |index| {
        HomeMessage::CategorySelected(index.to_string())
    });

    let top_bar = top_bar::view(
        &state.top_bar_active_tab,
        &state.top_bar_search_query,
        |tab| HomeMessage::TabSelected(tab),
        |query| HomeMessage::SearchChanged(query),
    );

    let status_bar = status_bar::view(&state.status_bar);

    let main_content: Element<'_, HomeMessage> = if state.loading {
        container(Spinner::default())
            .center_x(Fill)
            .center_y(Fill)
            .width(Fill)
            .height(Fill)
            .into()
    } else {
        // ── Featured section ────────────────────────────────────────────
        let featured_start = state
            .channel_data
            .len()
            .saturating_sub(10);
        let featured_count = state.channel_data.len().min(10);

        let featured_cards: Vec<Element<'_, HomeMessage>> = state
            .channel_data
            .iter()
            .skip(featured_start)
            .take(featured_count)
            .map(|ch| {
                let id = ch.id.clone();
                channel_card::view(ch, move |_| HomeMessage::ChannelClicked(id.clone()))
            })
            .collect();

        // ── Category tabs ──────────────────────────────────────────────
        let top_category_count = state.category_data.len().min(10);
        let cat_tabs = scrollable(category_tabs::view(
            &state.category_data,
            top_category_count,
            &state.active_category,
            |name| HomeMessage::CategorySelected(name),
        )).direction(Direction::Horizontal(Scrollbar::hidden()));

        // ── Popular section ─────────────────────────────────────────────
        let popular_count = state.channel_data.len().min(10);
        let popular_cards: Vec<Element<'_, HomeMessage>> = state
            .channel_data
            .iter()
            .take(popular_count)
            .map(|ch| {
                let id = ch.id.clone();
                channel_card::view(ch, move |_| HomeMessage::ChannelClicked(id.clone()))
            })
            .collect();

        let featured_section = column![
            text("Featured Channels")
                .size(theme::FONT_SIZE_XL)
                .color(theme::TEXT_PRIMARY)
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Font::DEFAULT
                }),
            scrollable(row(featured_cards).spacing(theme::SPACING_MD))
                .direction(Direction::Horizontal(Scrollbar::hidden()))
                .width(Fill)
                .height(theme::CHANNEL_CARD_HEIGHT + 20.0),
        ]
        .spacing(theme::SPACING_MD);

        let popular_section = column![
            text("Popular Channels")
                .size(theme::FONT_SIZE_XL)
                .color(theme::TEXT_PRIMARY)
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Font::DEFAULT
                }),
            scrollable(row(popular_cards).spacing(theme::SPACING_MD))
                .direction(Direction::Horizontal(Scrollbar::hidden()))
                .width(Fill)
                .height(theme::CHANNEL_CARD_HEIGHT + 20.0),
        ]
        .spacing(theme::SPACING_MD);

        let body = column![featured_section, cat_tabs, popular_section]
            .spacing(theme::SPACING_XL)
            .padding(theme::SPACING_XL)
            .align_x(Center);


        scrollable(body)
            .width(Fill)
            .height(Fill)
            .into()
    };

    let content = row![
        sidebar,
        container(main_content)
            .width(Fill)
            .height(Fill)
            .style(|_theme: &iced::Theme| {
                use iced::widget::container::Style;
                Style {
                    background: Some(iced::Background::Color(theme::BACKGROUND)),
                    ..Style::default()
                }
            }),
    ]
    .spacing(0)
    .width(Fill)
    .height(Fill);

    column![top_bar, content, status_bar]
        .spacing(0)
        .width(Fill)
        .height(Fill)
        .into()
}