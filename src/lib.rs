slint::include_modules!();

pub mod mappers;
pub mod controllers;
pub mod config;

pub mod utils;

pub mod player;

pub use crate::controllers::home_controller::HomeController;
pub use crate::controllers::player_controller::PlayerController;
pub use config::AppConfig;
pub use player::GstPlayerController;