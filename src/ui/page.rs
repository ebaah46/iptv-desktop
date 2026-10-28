use iced::Size;

/// Application screens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Home,
    Player,
    Country,
}


/// Routing data lives here rather than being written into another screen's state
/// before the route is dispatched.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Route {
    pub page: Page,
    pub focus: Option<String>,
}

impl Route {
    pub fn to(page: Page) -> Self {
        Self { page, focus: None }
    }

    /// Route that carries a selection to apply on entry.
    pub fn focused(page: Page, focus: impl Into<String>) -> Self {
        Self { page, focus: Some(focus.into()) }
    }
}


/// Window configuration constants.
pub const WINDOW_SIZE: Size = Size::new(1024.0, 720.0);
pub const WINDOW_TITLE: &str = "IPTV Desktop";