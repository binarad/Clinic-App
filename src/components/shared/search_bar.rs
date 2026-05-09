use crate::theme;
use iced::widget::{button, row, text, text_input};
use iced::{Alignment, Element, Length}; // Assuming your theme is available at crate root!

pub fn search_bar<'a, Message: Clone + 'a>(
    current_query: &str,
    placeholder: &str,
    on_change: impl Fn(String) -> Message + 'a,
    on_clear: Option<Message>,
) -> Element<'a, Message> {
    let input = text_input(placeholder, current_query)
        .on_input(on_change)
        .padding(10)
        .width(Length::Fill);

    let mut content = row![input]
        .spacing(10)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    // Only show the clear button if there is text, and a clear message was provided
    if let Some(clear_msg) = on_clear
        && !current_query.is_empty()
    {
        content = content.push(
            button(text("Clear"))
                .on_press(clear_msg)
                .style(theme::secondary_button) // Make sure this matches your theme
                .padding([10, 15]),
        );
    }

    content.into()
}
