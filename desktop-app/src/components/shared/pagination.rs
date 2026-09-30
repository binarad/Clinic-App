use crate::theme;
use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};

pub fn pagination<'a, Message: Clone + 'a>(
    current_page: usize,
    total_pages: usize,
    on_prev: Message,
    on_next: Message,
) -> Element<'a, Message> {
    // If we are on page 1, don't attach on_press (disables the button)
    let prev_btn = if current_page > 1 {
        button(text("← Previous"))
            .on_press(on_prev)
            .style(theme::secondary_button)
            .padding([8, 12])
    } else {
        button(text("← Previous"))
            .style(theme::secondary_button)
            .padding([8, 12])
    };

    // If we are on the last page, don't attach on_press (disables the button)
    let next_btn = if current_page < total_pages && total_pages > 0 {
        button(text("Next →"))
            .on_press(on_next)
            .style(theme::secondary_button)
            .padding([8, 12])
    } else {
        button(text("Next →"))
            .style(theme::secondary_button)
            .padding([8, 12])
    };

    let page_info = text(format!(
        "Page {} of {}",
        current_page,
        total_pages.max(1) // Prevent "Page 1 of 0" if list is empty
    ))
    .size(16);

    row![
        prev_btn,
        // Center the text taking up the remaining space
        container(page_info)
            .width(Length::Fill)
            .center_x(Length::Fill),
        next_btn
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .into()
}
