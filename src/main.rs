use iced::widget::{Column, button, container, row, table, text};
use iced::{Element, Font, Length};

use crate::components::sidebar::{self, Tab};
use crate::database::models::Employee;
// use crate::database::schema::employee::full_name;
use crate::db_test::{add_employee, establish_connection, show_employees};

// TODO: Move all mods to lib.rs
pub mod db_test;
pub mod theme;

pub mod components;
pub mod database;
pub mod views;

#[derive(Debug, Clone)]
enum Message {
    SidebarMessage(sidebar::Message),
    AddEmployee(String, Option<String>),
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
            Message::AddEmployee(full_name, phone) => {
                let conn = &mut establish_connection();
                // TODO
                add_employee(conn, &full_name, phone.as_deref());
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // Render the sidebar and map its local messages up to the global Message enum
        let sidebar_view = sidebar::view(&self.active_tab).map(Message::SidebarMessage);

        // Render the main content area dynamically based on the active tab
        let content_view: Element<Message> = match self.active_tab {
            Tab::Dashboard => text("Dashboard View").size(30).into(),
            Tab::Patients => text("Patients View").size(30).into(),
            // Tab::Employees => employees_list(&self.employees),
            Tab::Employees => employees_table(&self.employees),
            // Tab::Employees => widget::text("Employees View").size(30).into(),
            Tab::Appointments => text("Appointments View").size(30).into(),
            Tab::Registry => text("Registry View").size(30).into(),
        };

        // Wrap the content
        let main_content = container(content_view)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .style(theme::main_background);

        row![sidebar_view, main_content].into()
    }
}
fn main() -> iced::Result {
    // Testing DB
    // let conn = &mut establish_connection();
    // let full_name = String::from("Binarad Roberto");
    // let phone = String::from("+066666666");
    // add_employee(conn, &full_name, Some(&phone));
    // show_employees();

    iced::application(ClinicApp::new, ClinicApp::update, ClinicApp::view)
        .title("Clinic App")
        .run()
}

// ----------------------------------------------
fn employees_list<'a>(employees: &'a Vec<Employee>) -> Element<'a, Message>
where
    Message: 'a,
{
    // let name = String::from("bibajars Jubaris");
    let mut fullname_column: Column<'_, Message> = Column::new().spacing(10.0);
    for employ in employees {
        fullname_column = fullname_column.push(text(&employ.full_name).size(18.0));
        fullname_column = fullname_column
            .push(text(employ.phone.as_deref().unwrap_or("No phone provided")).size(12.0));
    }

    // TODO: Make it update the list after adding a new employee
    //
    // let btn = widget::button(widget::text("Add new"))
    //     .width(iced::Length::Fill)
    //     .padding(12)
    //     .on_press(Message::AddEmployee(name));
    // fullname_column = fullname_column.push(btn);
    container(fullname_column)
        .padding(20)
        .style(theme::white_card)
        .into()
}

pub fn employees_table<'a>(employees: &'a [Employee]) -> Element<'a, Message> {
    // 1. Define the Columns
    let columns = vec![
        // Column 1: ID
        table::column(
            text("ID").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, Message> {
                Element::from(text(employee.employee_id.to_string()))
            },
        )
        .width(Length::Fixed(50.0)),
        // Column 2: Full Name
        table::column(
            text("Full Name").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, Message> {
                Element::from(text(&employee.full_name))
            },
        )
        .width(Length::FillPortion(2)),
        // Column 3: Phone
        table::column(
            text("Phone").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, Message> {
                let phone_text = employee.phone.as_deref().unwrap_or("N/A");
                Element::from(text(phone_text))
            },
        )
        .width(Length::FillPortion(2)),
        // Column 4: Actions (The buttons on the right)
        table::column(
            text("Actions").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, Message> {
                let edit_btn = button("Edit");
                let delete_btn = button("Delete");

                row![edit_btn, delete_btn].spacing(5).into()
            },
        )
        .width(Length::Fixed(150.0)),
    ];

    // 2. Build the actual table using the columns and the raw data rows
    let data_table = table(columns, employees)
        .padding(10.0)
        .separator_y(1.0)
        .separator_x(0.0);

    // Wrap it in a container for styling
    container(data_table).width(Length::Fill).into()
}
