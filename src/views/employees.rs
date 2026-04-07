use iced::widget::{button, column, container, row, table, text, text_input};
use iced::{Alignment, Element, Length};

use iced_aw::drop_down;

#[derive(Debug, Clone)]
pub enum EmployeesMessage {
    SearchBarContentChanged(String),
}

// pub fn view() -> Element<'static, EmployeesMessage> {
//     let header = row![text("Employees").size(30.0)]
//         .align_y(Alignment::Center)
//         .width(Length::Fill);

//     let tools_row; // Add search_bar and the others dropdown and button

//     let
// }
