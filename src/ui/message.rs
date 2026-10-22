use crate::player::commands::PlayerCommand;
use crate::ui::page::Page;
use crate::ui::screens::home::HomeMessage;
use crate::ui::screens::player::PlayerMessage;
use crate::ui_model::LogoImage;

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
    /// A batch of channel logos fetched/decoded in the background, keyed by
    /// channel id. Delivered progressively so the grid fills in as downloads
    /// complete rather than all at once.
    LogoBatch(Vec<(String, Option<LogoImage>)>),
    /// Delegate to the home screen.
    Home(HomeMessage),
    /// Delegate to the player screen.
    Player(PlayerMessage),

    PlayerCommand(PlayerCommand),

    Unknown
}