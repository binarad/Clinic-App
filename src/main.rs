use iced::widget::{container, row, text};
use iced::{Element, Length};

use crate::components::sidebar::{self, Tab};
use crate::database::models::Employee;
use crate::database::operations::{
    delete_employee, fetch_employees_db, insert_employee_db, update_employee_db,
};
use crate::views::employees::{EmployeesMessage, FormField, add_employee_form};

// TODO: Move all mods to lib.rs
pub mod theme;

pub mod components;
pub mod database;
pub mod views;

#[derive(Debug, Clone, Default)]
pub struct DraftEmployee {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ActiveModal {
    AddEmployee(DraftEmployee),

    EditEmployee {
        target_id: i32,
        draft: DraftEmployee,
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
                EmployeesMessage::FieldChanged(field, new_value) => {
                    // Mutable reference to the draft whether we are adding or editing
                    let draft = match &mut self.active_modal {
                        Some(ActiveModal::AddEmployee(draft)) => draft,
                        Some(ActiveModal::EditEmployee { draft, .. }) => draft,
                        None => return iced::Task::none(),
                    };

                    match field {
                        FormField::Name => draft.name = new_value,
                        FormField::Phone => draft.phone = new_value,
                        FormField::Email => draft.email = new_value,
                        FormField::Role => draft.role = Some(new_value),
                    }

                    return iced::Task::none();
                }

                EmployeesMessage::DeleteEmployee(id) => {
                    return iced::Task::perform(delete_employee(id), move |result| {
                        Message::EmployeeView(EmployeesMessage::DeletedEmployee(result, id))
                    });
                }

                EmployeesMessage::DeletedEmployee(result, deleted_id) => {
                    match result {
                        Ok(_) => {
                            // Success! Update the UI by removing the employee from the vector.
                            // .retain() keeps only the employees whose ID does NOT match the deleted_id.
                            self.employees.retain(|emp| emp.employee_id != deleted_id);
                        }
                        Err(e) => {
                            println!("Failed to delete employee: {}", e);
                        }
                    }
                }

                EmployeesMessage::OpenAddForm => {
                    self.active_modal = Some(ActiveModal::AddEmployee(DraftEmployee::default()));
                }

                EmployeesMessage::OpenEditForm(emp) => {
                    self.active_modal = Some(ActiveModal::EditEmployee {
                        target_id: emp.employee_id,
                        draft: DraftEmployee {
                            name: emp.full_name,
                            phone: emp.phone.unwrap_or_default(),
                            email: emp.email.unwrap_or_default(),
                            role: Some(emp.role),
                        },
                    });
                }

                EmployeesMessage::CloseAddForm => {
                    self.active_modal = None;
                }

                EmployeesMessage::SubmitForm => {
                    // Extract the data from the modal state
                    match &self.active_modal {
                        Some(ActiveModal::AddEmployee(draft)) => {
                            // Basic Validation: Ensure they picked a role and entered a name
                            if draft.name.trim().is_empty() || draft.role.is_none() {
                                println!("Validation failed: Name and Role are required.");
                                return iced::Task::none(); // You could show a UI error here instead
                            }

                            // Clone the strings to send them to the background thread
                            let role = draft.role.clone().unwrap();
                            let name = draft.name.clone();
                            let phone = draft.phone.clone();
                            let email = draft.email.clone();

                            // Lock the UI
                            self.is_saving = true;

                            // Tell Iced to run the DB function and return the EmployeeAdded message when done
                            return iced::Task::perform(
                                insert_employee_db(role, name, phone, email),
                                |result| {
                                    Message::EmployeeView(EmployeesMessage::EmployeeAdded(result))
                                },
                            );
                        }

                        Some(ActiveModal::EditEmployee { target_id, draft }) => {
                            if draft.name.trim().is_empty() || draft.role.is_none() {
                                return iced::Task::none();
                            }

                            let id = *target_id;
                            let role = draft.role.clone().unwrap();
                            let name = draft.name.clone();
                            let phone = draft.phone.clone();
                            let email = draft.email.clone();

                            self.is_saving = true;

                            return iced::Task::perform(
                                update_employee_db(id, role, name, phone, email),
                                |result| {
                                    Message::EmployeeView(EmployeesMessage::EmployeeUpdated(result))
                                },
                            );
                        }

                        None => return iced::Task::none(),
                    }
                }

                EmployeesMessage::EmployeeUpdated(result) => {
                    self.is_saving = false;

                    match result {
                        Ok(updated_employee) => {
                            // Find the old employee in the vector and replace it with the new one
                            if let Some(index) = self
                                .employees
                                .iter()
                                .position(|e| e.employee_id == updated_employee.employee_id)
                            {
                                self.employees[index] = updated_employee;
                            }
                            self.active_modal = None
                        }
                        Err(e) => println!("Failed to update: {}", e),
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
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // Render the sidebar and map its local messages up to the global Message enum
        let sidebar_view = sidebar::view(&self.active_tab).map(Message::Sidebar);
        // 1. Check if a Modal is open. If so, render the form and map its messages.
        if let Some(modal) = &self.active_modal {
            let draft = match modal {
                ActiveModal::AddEmployee(d) => d,
                ActiveModal::EditEmployee { draft: d, .. } => d,
            };

            let employee_form = container(add_employee_form(
                &draft.name,
                &draft.phone,
                &draft.email,
                draft.role.as_deref(),
            ))
            .width(Length::Fill)
            .height(Length::Fill);

            return Element::from(employee_form).map(Message::EmployeeView);
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
