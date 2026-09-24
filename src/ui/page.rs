use iced::Size;

/// Application screens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Home,
    Player,
}

/// Window configuration constants.
pub const WINDOW_SIZE: Size = Size::new(1024.0, 720.0);
pub const WINDOW_TITLE: &str = "IPTV Desktop";