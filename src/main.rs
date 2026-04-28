use iced::widget::{container, row, text};
use iced::{Element, Length};

use crate::components::sidebar::{self, Tab};
use crate::database::models::Employee;
use crate::database::operations::{fetch_employees_db, insert_employee_db};
use crate::views::employees::{self, EmployeesMessage};

// TODO: Move all mods to lib.rs
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
    EmployeeView(EmployeesMessage),
    // CloseModal,
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
            employees: fetch_employees_db(),
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

                EmployeesMessage::SubmitForm => {
                    // Extract the data from the modal state
                    if let Some(ActiveModal::AddEmployee {
                        draft_name,
                        draft_phone,
                        draft_email,
                        draft_role,
                    }) = &self.active_modal
                    {
                        // Basic Validation: Ensure they picked a role and entered a name
                        if draft_name.trim().is_empty() || draft_role.is_none() {
                            println!("Validation failed: Name and Role are required.");
                            return iced::Task::none(); // You could show a UI error here instead
                        }

                        // Clone the strings to send them to the background thread
                        let role = draft_role.clone().unwrap();
                        let name = draft_name.clone();
                        let phone = draft_phone.clone();
                        let email = draft_email.clone();

                        // Lock the UI
                        self.is_saving = true;

                        // Tell Iced to run the DB function and return the EmployeeAdded message when done
                        return iced::Task::perform(
                            insert_employee_db(role, name, phone, email),
                            |result| Message::EmployeeView(EmployeesMessage::EmployeeAdded(result)),
                        );
                    }
                }

                EmployeesMessage::EmployeeAdded(result) => {
                    self.is_saving = false; // Unlock the UI

                    match result {
                        Ok(new_employee) => {
                            // Success! Add it to our local list so the table updates
                            self.employees.push(new_employee);
                            // Close the modal, which clears the draft data automatically
                            self.active_modal = None;
                        }
                        Err(e) => {
                            // DB failed (e.g. unique constraint error)
                            println!("Failed to add employee: {}", e);
                            // The modal stays open so they can fix their mistake
                        }
                    }
                }
            },
            // Message::CloseModal => {
            //     self.active_modal = None;
            // }
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
            Tab::Employees => views::employees::view(&self.employees).map(Message::EmployeeView),
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
