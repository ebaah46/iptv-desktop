pub mod card {
    use std::sync::Arc;

    use iced::widget::{button, column, container, row, text};
    use iced::{Element, Fill, Font};
    use iced::advanced::text::Wrapping;
    use iced::advanced::text::Wrapping::Word;
    use crate::ui::theme;
    use crate::ui_model::CountryItem;

    /// Renders a single country card.
    ///
    /// `selected` drives the accent border; the app owns that decision
    /// (`CountryState::selected_country`), the card only reflects it.
    pub fn view<Message: 'static + Clone>(
        item: Arc<CountryItem>,
        selected: bool,
        on_selected: impl Fn(String) -> Message + 'static + Clone,
    ) -> Element<'static, Message> {
        let code = item.code().to_string();

        // The "flag" is a glyph (emoji string) the catalog already provides —
        // no async load, no pipeline. Empty glyph degrades to a blank box.
        let flag: Element<'static, Message> = container(
            text(item.flag().to_string())
                .size(theme::COUNTRY_FLAG_GLYPH)
                .color(theme::TEXT_PRIMARY),
        )
            .width(theme::COUNTRY_FLAG_GLYPH + theme::SPACING_SM)
            .height(theme::COUNTRY_FLAG_GLYPH)
            .center_x(Fill)
            .center_y(Fill)
            .into();

        let name = text(item.name().to_string())
            .size(theme::FONT_SIZE_LG)
            .wrapping(Word)
            .color(theme::TEXT_PRIMARY)
            .font(Font {
                weight: iced::font::Weight::Semibold,
                ..Font::DEFAULT
            });

        // Two languages max keeps every card the same height; the rest is a tooltip
        // concern, not a layout concern.
        let languages = item.languages().iter().take(2).cloned().collect::<Vec<_>>().join(" · ");
        let meta = text(if languages.is_empty() { code.clone() } else { languages })
            .size(theme::FONT_SIZE_MD)
            .color(theme::TEXT_SECONDARY)
            .wrapping(Word)
            .width(Fill);

        let body = row![flag, column![name, meta].spacing(theme::SPACING_XS)]
            .spacing(theme::SPACING_SM)
            .align_y(iced::Center)
            .padding(theme::SPACING_MD);

        button(
            container(body)
                .width(theme::COUNTRY_CARD_WIDTH)
                .height(theme::COUNTRY_CARD_HEIGHT)
                .align_y(iced::Center),
        )
            .on_press(on_selected(code))
            .padding(0)
            .style(move |_theme: &iced::Theme, status| {
                use iced::widget::button::{Status, Style};
                let background = match status {
                    Status::Hovered => Some(iced::Background::Color(theme::SURFACE_HOVER)),
                    Status::Pressed => Some(iced::Background::Color(theme::ACCENT)),
                    _ => Some(iced::Background::Color(theme::CARD_BG)),
                };
                Style {
                    background,
                    border: iced::Border {
                        width: if selected { 2.0 } else { 1.5 },
                        color: if selected { theme::ACCENT } else { theme::BORDER_COLOR },
                        radius: theme::RADIUS_MD.into(),
                    },
                    ..Style::default()
                }
            })
            .into()
    }
}

pub mod grid {
    use std::sync::Arc;

    use iced::widget::{column, row, scrollable};
    use iced::{Element, Fill};

    use crate::ui::components::country::card;
    use crate::ui::theme;
    use crate::ui_model::{CountryItem, CountryItems};

    /// Renders countries in a fixed-column grid layout.
    ///
    /// `columns` — number of columns per row (clamped to >= 1)
    /// `spacing` — gap between cards (horizontal and vertical)
    /// `selected` — code of the currently selected country, drawn with an accent border
    /// `on_click` — callback invoked with the country code when a card is clicked
    pub fn view<Message: 'static + Clone>(
        countries: CountryItems,
        columns: usize,
        spacing: f32,
        selected: Option<String>,
        on_click: impl Fn(String) -> Message + 'static + Clone,
    ) -> Element<'static, Message> {
        // `slice::chunks(0)` panics, and a zero-width container would silently do the
        // same here — channel_grid has this hazard too, so guard it in both if you copy.
        let columns = columns.max(1);

        // Pre-compute owned data so no closure captures the borrow of `countries`.
        let rows: Vec<Element<'static, Message>> = countries
            .chunks(columns)
            .map(|chunk| {
                let chunk_data: Vec<(String, Arc<CountryItem>)> = chunk
                    .iter()
                    .map(|item| (item.code().to_string(), item.clone()))
                    .collect();

                let mut row_children: Vec<Element<'static, Message>> =
                    Vec::with_capacity(chunk_data.len());
                for (code, item) in chunk_data {
                    let cb = on_click.clone();
                    let is_selected = selected.as_deref() == Some(code.as_str());
                    row_children.push(card::view(item, is_selected, move |c| cb(c)));
                }
                row(row_children).spacing(spacing).into()
            })
            .collect();

        column(rows).spacing(spacing).into()
    }

    /// Renders countries in a scrollable grid layout.
    pub fn scrollable_view<Message: 'static + Clone>(
        countries: CountryItems,
        columns: usize,
        spacing: f32,
        selected: Option<String>,
        on_click: impl Fn(String) -> Message + 'static + Clone,
    ) -> Element<'static, Message> {
        scrollable(view(countries, columns, spacing, selected, on_click))
            .width(Fill)
            .height(Fill)
            .into()
    }

    /// Computes the column count from the available width using COUNTRY_CARD_WIDTH.
    pub fn auto_view<Message: 'static + Clone>(
        countries: CountryItems,
        available_width: f32,
        spacing: f32,
        selected: Option<String>,
        on_click: impl Fn(String) -> Message + 'static + Clone,
    ) -> Element<'static, Message> {
        let card_width = theme::COUNTRY_CARD_WIDTH;
        let columns = ((available_width + spacing) / (card_width + spacing)).max(1.0) as usize;
        view(countries, columns, spacing, selected, on_click)
    }
}

