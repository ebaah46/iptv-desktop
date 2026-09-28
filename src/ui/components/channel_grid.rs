use iced::widget::{column, row, scrollable, Scrollable};
use iced::{Element, Fill};
use libcore::domain::Channel;

use crate::ui::components::channel_card;
use crate::ui::theme;

/// Renders channels in a responsive grid layout.
///
/// `channels` — the list of channels to display  
/// `columns` — number of columns per row  
/// `spacing` — gap between cards (horizontal and vertical)  
/// `on_click` — callback invoked with the channel id when a card is clicked
pub fn view<Message: 'static + Clone>(
    channels: &[Channel],
    columns: usize,
    spacing: f32,
    on_click: impl Fn(String) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    // Pre-compute shared data so we don't capture `on_click` inside FnMut closures.
    let rows: Vec<Element<'static, Message>> = channels
        .chunks(columns)
        .map(|chunk| {
            let chunk_data: Vec<(String, &Channel)> =
                chunk.iter().map(|ch| (ch.id.clone(), ch)).collect();
            let mut row_children: Vec<Element<'static, Message>> = Vec::with_capacity(chunk_data.len());
            for (id, ch) in chunk_data {
                let cb = on_click.clone();
                row_children.push(channel_card::view(ch, move |_| cb(id.clone())));
            }
            row(row_children).spacing(spacing).into()
        })
        .collect();

    column(rows).spacing(spacing).into()
}

/// Renders channels in a scrollable responsive grid layout.
///
/// Same parameters as [`view`], but wraps the grid in a vertical scrollable.
pub fn scrollable_view<Message: 'static + Clone>(
    channels: &[Channel],
    columns: usize,
    spacing: f32,
    on_click: impl Fn(String) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    scrollable(view(channels, columns, spacing, on_click))
        .width(Fill)
        .height(Fill)
        .into()
}

/// Convenience wrapper that uses the default CHANNEL_CARD_WIDTH to compute the column count.
///
/// `available_width` — the width available for the grid (used to calculate columns).
pub fn auto_view<Message: 'static + Clone>(
    channels: &[Channel],
    available_width: f32,
    spacing: f32,
    on_click: impl Fn(String) -> Message + 'static + Clone,
) -> Element<'static, Message> {
    let card_width = theme::CHANNEL_CARD_WIDTH;
    // Ensure minimum 1 column, compute based on available space.
    let columns = ((available_width + spacing) / (card_width + spacing)).max(1.0) as usize;
    view(channels, columns, spacing, on_click)
}