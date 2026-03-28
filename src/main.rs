use iced::widget;

pub mod models;
pub mod schema;

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

struct ClinicApp {
    count: i32,
}

impl ClinicApp {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let row = widget::row![
            widget::button("-").on_press(Message::Decrement),
            widget::text!("Count: {}", self.count),
            widget::button("+").on_press(Message::Increment)
        ]
        .spacing(10);

        widget::container(row).center(iced::Length::Fill).into()
    }
}
fn main() -> iced::Result {
    iced::application(ClinicApp::new, ClinicApp::update, ClinicApp::view)
        .title("Clinic App")
        .run()
}
