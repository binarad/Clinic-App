use iced::widget::{container, row, text};
use iced::{Element, Length};

use crate::components::sidebar::{self, Tab};
use crate::database::models::Employee;
// use crate::database::schema::employee::full_name;
use crate::db_test::{add_employee, establish_connection, show_employees};
use crate::views::employees::{self, EmployeesMessage};
// use crate::views;

// TODO: Move all mods to lib.rs
pub mod db_test;
pub mod theme;

pub mod components;
pub mod database;
pub mod views;

#[derive(Debug, Clone)]
pub enum ActiveModal {
    AddEmployee {
        draft_name: String,
        draft_phone: String,
        draft_email: String,
        draft_role: Option<String>,
    },
}

#[derive(Debug, Clone)]
enum Message {
    Sidebar(sidebar::Message),
    AddEmployee(String, Option<String>, String, Option<String>),
    EmployeeView(EmployeesMessage),
    CloseModal,
}

pub struct ClinicApp {
    pub employees: Vec<Employee>,
    pub active_tab: Tab,
    pub active_modal: Option<ActiveModal>,
    pub is_saving: bool, // Locks UI during database insertion
}

impl ClinicApp {
    fn new() -> Self {
        Self {
            employees: show_employees(),
            active_tab: Tab::Dashboard,
            active_modal: None,
            is_saving: false,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Sidebar(sidebar::Message::SelectedTab(new_tab)) => {
                self.active_tab = new_tab;
            }
            Message::AddEmployee(full_name, phone, role, email) => {
                let conn = &mut establish_connection();
                // TODO
                add_employee(conn, &full_name, phone.as_deref(), &role, email.as_deref());
            }

            Message::EmployeeView(employee_msg) => match employee_msg {
                EmployeesMessage::OpenAddForm => {
                    self.active_modal = Some(ActiveModal::AddEmployee {
                        draft_name: String::new(),
                        draft_phone: String::new(),
                        draft_email: String::new(),
                        draft_role: None,
                    });
                }
                EmployeesMessage::RoleSelected(new_role) => {
                    // Check if the AddEmployee modal is currently open
                    if let Some(ActiveModal::AddEmployee { draft_role, .. }) =
                        &mut self.active_modal
                    {
                        // Update the state!
                        *draft_role = Some(new_role);
                    }
                }

                EmployeesMessage::NameChanged(new_name) => {
                    if let Some(ActiveModal::AddEmployee { draft_name, .. }) =
                        &mut self.active_modal
                    {
                        *draft_name = new_name;
                    }
                }

                EmployeesMessage::PhoneChanged(new_phone) => {
                    if let Some(ActiveModal::AddEmployee { draft_phone, .. }) =
                        &mut self.active_modal
                    {
                        *draft_phone = new_phone;
                    }
                }

                EmployeesMessage::EmailChanged(new_email) => {
                    if let Some(ActiveModal::AddEmployee { draft_email, .. }) =
                        &mut self.active_modal
                    {
                        *draft_email = new_email;
                    }
                }

                EmployeesMessage::CloseAddForm => {
                    self.active_modal = None;
                }

                _ => {}
            },
            Message::CloseModal => {
                self.active_modal = None;
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // Render the sidebar and map its local messages up to the global Message enum
        let sidebar_view = sidebar::view(&self.active_tab).map(Message::Sidebar);
        // 1. Check if a Modal is open. If so, render the form and map its messages.
        if let Some(modal) = &self.active_modal {
            match modal {
                ActiveModal::AddEmployee {
                    draft_name,
                    draft_phone,
                    draft_email,
                    draft_role,
                } => {
                    let form_container = container(employees::add_employee_form(
                        draft_name,
                        draft_phone,
                        draft_email,
                        draft_role.as_deref(),
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill);

                    return Element::from(form_container).map(Message::EmployeeView);
                }
            }
        }
        // Render the main content area dynamically based on the active tab
        let content_view: Element<Message> = match self.active_tab {
            Tab::Dashboard => text("Dashboard View").size(30).into(),
            Tab::Patients => text("Patients View").size(30).into(),
            // Tab::Employees => employees_list(&self.employees),
            Tab::Employees => views::employees::view(&self.employees).map(Message::EmployeeView),
            // Tab::Employees => employees_table(&self.employees),
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
    iced::application(ClinicApp::new, ClinicApp::update, ClinicApp::view)
        .title("Clinic App")
        .run()
}
