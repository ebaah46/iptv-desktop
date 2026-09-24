use crate::ui::page::Page;
use crate::ui::screens::home::HomeMessage;
use crate::ui::screens::player::PlayerMessage;

/// Messages handled by the application.
#[derive(Debug, Clone, Default)]
pub enum Message {
    /// Navigate to the given page.
    NavigateTo(Page),
    #[default]
    /// Prepare data for homescreen
    PrepareHome,
    /// Notify that data for homescreen is ready
    HomeReady,
    /// Delegate to the home screen.
    Home(HomeMessage),
    /// Delegate to the player screen.
    Player(PlayerMessage),

    Unknown
}