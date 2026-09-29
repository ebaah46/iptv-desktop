use iced::Color;

// ── Backgrounds ──────────────────────────────────────────────────────
pub const BACKGROUND: Color = Color::from_rgb(0.075, 0.075, 0.102);   // #13131a
pub const SURFACE: Color = Color::from_rgb(0.118, 0.118, 0.180);      // #1e1e2e
pub const SURFACE_HOVER: Color = Color::from_rgb(0.165, 0.165, 0.243); // #2a2a3e
pub const CARD_BG: Color = Color::from_rgb(0.102, 0.102, 0.180);      // #1a1a2e
pub const SIDEBAR_BG: Color = Color::from_rgb(0.094, 0.094, 0.145);   // #181825
pub const TOPBAR_BG: Color = Color::from_rgb(0.078, 0.078, 0.122);    // #14141f
pub const STATUSBAR_BG: Color = Color::from_rgb(0.055, 0.055, 0.094); // #0e0e18

// ── Accents / interactive ───────────────────────────────────────────
pub const PRIMARY: Color = Color::from_rgb(0.102, 0.451, 0.910);      // #1a73e8
pub const PRIMARY_HOVER: Color = Color::from_rgb(0.084, 0.341, 0.690);// #1557b0
pub const ACCENT: Color = Color::from_rgb(1.000, 0.420, 0.208);       // #ff6b35
pub const ACCENT_HOVER: Color = Color::from_rgb(0.878, 0.345, 0.125); // #e05520
pub const SUCCESS: Color = Color::from_rgb(0.298, 0.686, 0.314);       // #4caf50

// ── Text ────────────────────────────────────────────────────────────
pub const TEXT_PRIMARY: Color = Color::WHITE;
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.627, 0.627, 0.690); // #a0a0b0

// ── Border ──────────────────────────────────────────────────────────
pub const BORDER_COLOR: Color = Color::from_rgb(0.165, 0.165, 0.243); // #2a2a3e

// ── Progress ────────────────────────────────────────────────────────
pub const PROGRESS_BG: Color = Color::from_rgb(0.165, 0.165, 0.243);  // #2a2a3e
pub const PROGRESS_FILL: Color = Color::from_rgb(0.102, 0.451, 0.910);// #1a73e8

// ── Spacing ─────────────────────────────────────────────────────────
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;

// ── Border-radius ───────────────────────────────────────────────────
pub const RADIUS_SM: f32 = 6.0;
pub const RADIUS_MD: f32 = 10.0;
pub const RADIUS_LG: f32 = 16.0;

// ── Font sizes ──────────────────────────────────────────────────────
pub const FONT_SIZE_SM: f32 = 11.0;
pub const FONT_SIZE_MD: f32 = 13.0;
pub const FONT_SIZE_LG: f32 = 15.0;
pub const FONT_SIZE_XL: f32 = 20.0;

// ── Component sizing ────────────────────────────────────────────────
pub const SIDEBAR_WIDTH: f32 = 180.0;
pub const TOPBAR_HEIGHT: f32 = 52.0;
pub const STATUSBAR_HEIGHT: f32 = 48.0;
pub const CHANNEL_CARD_WIDTH: f32 = 150.0;
pub const CHANNEL_CARD_HEIGHT: f32 = 100.0;