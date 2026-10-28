use std::sync::Arc;
use iced::{Element, Fill, Task};
use iced::widget::{button, container, text, column};
use libcore::facade::IptvFacade;
use crate::ui::components::country;
use crate::ui::theme;
use crate::ui_model::{CountryItem, CountryItems};

/// Messages handled by the Country screen
#[derive(Debug, Clone)]
pub enum CountryMessage {
    CountrySelected(String),
    BackRequested,
    Entered,
}


#[derive(Debug, Default)]
pub struct CountryState {
    pub selected_country: String,
    pub countries: CountryItems
}



pub fn update(state: &mut CountryState, message: CountryMessage, facade: Arc<IptvFacade>) -> Task<CountryMessage>{
    match message { CountryMessage::CountrySelected(_) => Task::none(),
        CountryMessage::BackRequested => Task::none(),
        CountryMessage::Entered => {
            Task::none()
        }
    }
}

pub fn view(state: &CountryState) -> Element<'_, CountryMessage> {
    let back_btn = button(
        text("← Back").size(theme::FONT_SIZE_MD).color(theme::TEXT_PRIMARY)
    )
        .padding([8, 16])
        .on_press(CountryMessage::BackRequested);

    let grid = country::grid::scrollable_view(
        state.countries.clone(),
        5,
        theme::SPACING_MD,
        (!state.selected_country.is_empty()).then(|| state.selected_country.clone()),
        |code| CountryMessage::CountrySelected(code),
    );

    container(
        column![
            back_btn,
            text("Available Countries").size(theme::FONT_SIZE_XL).color(theme::TEXT_PRIMARY),
            grid,
        ]   .height(Fill)
            .spacing(theme::SPACING_MD)
            .padding(theme::SPACING_XL),
    )
        .width(Fill)
        .height(Fill)
        .style(|_theme: &iced::Theme| {
            use iced::widget::container::Style;
            Style {
                background: Some(iced::Background::Color(theme::BACKGROUND)),
                ..Style::default()
            }
        })
        .into()

}