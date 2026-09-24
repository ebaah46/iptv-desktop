use crate::ui::page::Page;

/// Messages handled by the application.
#[derive(Debug, Clone)]
pub enum Message {
    /// Navigate to the given page.
    NavigateTo(Page),
    /// A channel was selected (by id).
    ChannelSelected(String),
    /// A category was selected (by id or "All").
    CategorySelected(String),
    /// Play/Pause toggled.
    PlayPauseToggled(bool),
    /// Back button pressed on the player screen.
    BackRequested,
}