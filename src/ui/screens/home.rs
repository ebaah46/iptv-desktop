use std::sync::Arc;

use iced::widget::{column, container, row, scrollable, text};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::{Center, Element, Fill, Font, Task};
use iced_aw::widgets::spinner::Spinner;
use libcore::domain::{Categories, Channels, Countries};
use libcore::facade::IptvFacade;
use crate::ui::components::category_tabs;
use crate::ui::components::channel_card;
use crate::ui::components::channel_grid;
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
    SearchResultsReady(String, Channels),
    DebounceExpired(u64, String),
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
    pub channel_data: Arc<Channels>,
    pub category_data: Categories,
    pub countries_data: Countries,
    pub loading: bool,
    pub active_category: String,
    /// Channels matching the current search query (empty if no search active).
    pub search_results: Channels,
    /// pending_search prevents race conditions when user types quickly
    pending_search: String,
    /// search id for identifying each query string on each key stroke
    search_id: u64,
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
    facade: Arc<IptvFacade>,
) -> Task<HomeMessage> {
    match message {
        HomeMessage::CategorySelected(name) => {
            if let Some(index) = state.sidebar.items.iter().position(|i| *i == name) {
                state.sidebar.active_index = index;
            }
            state.active_category = name;
            Task::none()
        }
        HomeMessage::TabSelected(tab) => {
            state.top_bar_active_tab = tab;
            Task::none()
        }
        HomeMessage::SearchChanged(query) => {
            let trimmed = query.trim().to_ascii_lowercase();
            state.top_bar_search_query = query;
            state.pending_search = trimmed.clone();

            // Increment search_id so the older pending debounce becomes invalid
            state.search_id += 1;
            let current_id = state.search_id;

            // Wait 200ms off the main thread before dispatching the search task
            Task::perform(
                async move {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    current_id
                },
                move |id| HomeMessage::DebounceExpired(id, trimmed),
            )
        }

        HomeMessage::DebounceExpired(id, query) => {
            if id != state.search_id {
                return Task::none();
            }

            // Spawn background thread to filter
            let data = state.channel_data.clone();
            let query_string = query.clone();
            Task::perform(
                tokio::task::spawn_blocking(move || {
                    let results: Channels = data
                        .iter()
                        .filter(|ch| {
                            ch.name.to_ascii_lowercase().contains(&query_string)
                                || ch.alt_names.iter().any(|a| a.to_ascii_lowercase().contains(&query_string))
                        })
                        .cloned()
                        .collect();
                    HomeMessage::SearchResultsReady(query, results)
                }),
                |res| res.unwrap_or_else(|_| HomeMessage::SearchResultsReady("".into(), vec![])),
            )
        }
        HomeMessage::SearchResultsReady(query, results) => {
            // Ignore stale results if a newer search was started
            // this prevents data races so we can avoid locks
            if state.pending_search == query {
                state.search_results = results;
            }
            Task::none()
        }
        HomeMessage::ChannelClicked(_id) => {
            // Navigation to player will be handled at the app level.
            Task::none()
        }
    }
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
    } else if !state.top_bar_search_query.trim().is_empty() {
        // ── Search results view ─────────────────────────────────────────
        let results = state.search_results.clone();
        let query = state.top_bar_search_query.clone();

        let heading = text(format!("Search results for \"{}\"", &query))
            .size(theme::FONT_SIZE_XL)
            .color(theme::TEXT_PRIMARY)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Font::DEFAULT
            });

        let result_count = text(format!("{} channels found", results.len()))
            .size(theme::FONT_SIZE_MD)
            .color(theme::TEXT_SECONDARY);

        // Use 5 columns for the grid — adjust as desired.
        let grid = channel_grid::view(
            results,
            5,
            theme::SPACING_MD,
            |id| HomeMessage::ChannelClicked(id),
        );

        let body = column![
            heading,
            result_count,
            grid,
        ]
        .spacing(theme::SPACING_MD)
        .padding(theme::SPACING_XL)
        .align_x(Center);

        scrollable(body)
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
                channel_card::view(ch.clone(), move |_| HomeMessage::ChannelClicked(id.clone()))
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
                channel_card::view(ch.clone(), move |_| HomeMessage::ChannelClicked(id.clone()))
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