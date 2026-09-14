slint::include_modules!();

pub mod mappers;
pub mod controllers;
pub mod config;

pub mod utils;

pub mod player;

pub use controllers::HomeController;
pub use config::AppConfig;
pub use player::GstPlayerController;