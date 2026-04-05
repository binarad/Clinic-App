use iced::Element;
use iced::widget;

use crate::components::sidebar::{self, Tab};
use crate::database::models::Employee;
use crate::db_test::show_employees;

// TODO: Move all mods to lib.rs
pub mod db_test;
pub mod theme;

pub mod components;
pub mod database;

#[derive(Debug, Clone)]
enum Message {
    SidebarMessage(sidebar::Message),
}

struct ClinicApp {
    employees: Vec<Employee>,
    active_tab: Tab,
}

impl ClinicApp {
    fn new() -> Self {
        Self {
            employees: show_employees(),
            active_tab: Tab::Dashboard,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::SidebarMessage(sidebar::Message::SelectedTab(new_tab)) => {
                self.active_tab = new_tab;
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // Render the sidebar and map its local messages up to the global Message enum
        let sidebar_view = sidebar::view(&self.active_tab).map(Message::SidebarMessage);

        // Render the main content area dynamically based on the active tab
        let content_view: Element<Message> = match self.active_tab {
            Tab::Dashboard => widget::text("Dashboard View").size(30).into(),
            Tab::Patients => widget::text("Patients View").size(30).into(),
            Tab::Employees => employees_list(&self.employees),
            // Tab::Employees => widget::text("Employees View").size(30).into(),
            Tab::Appointments => widget::text("Appointments View").size(30).into(),
            Tab::Registry => widget::text("Registry View").size(30).into(),
        };

        // Wrap the content
        let main_content = widget::container(content_view)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .style(theme::main_background);

        widget::row![sidebar_view, main_content].into()
    }
}
fn main() -> iced::Result {
    // Testing DB
    // let conn = &mut establish_connection();
    // let full_name = String::from("Ibragim");
    // let phone = String::from("+379569956");
    // add_employee(conn, &full_name, Some(&phone));
    // show_employees();

    iced::application(ClinicApp::new, ClinicApp::update, ClinicApp::view)
        .title("Clinic App")
        .run()
}

fn employees_list<'a, Message>(employees: &'a Vec<Employee>) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut fullname_column: widget::Column<'_, Message> = widget::Column::new().spacing(10.0);
    for employ in employees {
        fullname_column = fullname_column.push(widget::text(&employ.full_name).size(18.0));
        fullname_column = fullname_column
            .push(widget::text(employ.phone.as_deref().unwrap_or("No phone provided")).size(12.0));
    }
    widget::container(fullname_column)
        .padding(20)
        .style(theme::white_card)
        .into()
}
