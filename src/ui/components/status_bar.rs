use iced::widget::{container, row, text};
use iced::{Element, Fill};

use crate::ui::theme;

/// Data displayed by the status bar.
#[derive(Debug, Clone)]
pub struct StatusBarData {
    pub now_playing: String,
    pub has_content: bool,
    pub app_version: String,
    pub core_version: String,
    pub channel_count: usize,
    pub stream_quality: String,
}

impl Default for StatusBarData {
    fn default() -> Self {
        Self {
            now_playing: "Nothing playing".into(),
            has_content: false,
            app_version: "v0.1.0".into(),
            core_version: "v0.1.0".into(),
            channel_count: 0,
            stream_quality: "HD".into(),
        }
    }
}

/// Renders the status bar.
pub fn view<Message: 'static + Clone>(
    state: &StatusBarData,
) -> Element<'static, Message> {
    let has_content = state.has_content;
    let now_playing = state.now_playing.clone();
    let stream_quality = state.stream_quality.clone();
    let app_version = state.app_version.clone();
    let core_version = state.core_version.clone();
    let channel_count = state.channel_count;

    // ── Status dot ─────────────────────────────────────────────────────
    let dot = container(text(""))
        .width(8.0)
        .height(8.0)
        .style(move |_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(if has_content {
                    theme::SUCCESS
                } else {
                    theme::TEXT_SECONDARY
                })),
                ..Style::default()
            }
        });

    // ── Left section ───────────────────────────────────────────────────
    let left = row![
        dot,
        text(if has_content { "Now Playing:" } else { "Standby" })
            .size(theme::FONT_SIZE_SM)
            .color(theme::TEXT_SECONDARY),
        text(now_playing)
            .size(theme::FONT_SIZE_MD)
            .color(theme::TEXT_PRIMARY),
    ]
    .spacing(theme::SPACING_SM)
    .align_y(iced::alignment::Vertical::Center);

    // ── Badge helper ───────────────────────────────────────────────────
    let badge_txt = |label: String| -> Element<'static, Message> {
        container(
            text(label)
                .size(theme::FONT_SIZE_SM)
                .color(theme::TEXT_SECONDARY),
        )
        .padding([0.0, 8.0])
        .height(22.0)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::SURFACE)),
                border: iced::Border {
                    width: 1.0,
                    color: theme::BORDER_COLOR,
                    radius: theme::RADIUS_SM.into(),
                },
                ..Style::default()
            }
        })
        .into()
    };

    // ── Right section ──────────────────────────────────────────────────
    let right: Element<'static, Message> = row![
        badge_txt(stream_quality.clone()),
        badge_txt(format!("{} Ch", channel_count)),
        badge_txt(format!("App {}", app_version)),
        badge_txt(format!("Core {}", core_version)),
        container(text("🔊").size(12.0))
            .width(28.0)
            .height(22.0)
            .center_x(Fill)
            .center_y(Fill)
            .style(|_theme: &iced::Theme| {
                use iced::widget::container::Style;
                Style {
                    background: Some(iced::Background::Color(theme::SURFACE)),
                    ..Style::default()
                }
            }),
    ]
    .spacing(theme::SPACING_SM)
    .align_y(iced::alignment::Vertical::Center)
    .into();

    // ── Spacer ─────────────────────────────────────────────────────────
    let spacer: Element<'static, Message> =
        iced::widget::Space::new().width(Fill).height(Fill).into();

    // ── Assemble status bar ────────────────────────────────────────────
    let bar = row![left, spacer, right]
        .spacing(theme::SPACING_MD)
        .padding([0.0, theme::SPACING_LG])
        .align_y(iced::alignment::Vertical::Center)
        .height(theme::STATUSBAR_HEIGHT)
        .width(Fill);

    container(bar)
        .width(Fill)
        .height(theme::STATUSBAR_HEIGHT)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::STATUSBAR_BG)),
                ..Style::default()
            }
        })
        .into()
}