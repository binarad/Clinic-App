use iced::widget::{container, row, text};
use iced::{Element, Length};

use crate::components::sidebar::{self, Tab};
use crate::database::models::{Employee, Patient}; // Added Patient here!
use crate::database::operations::{
    delete_employee,
    delete_patient_db,
    edit_patient_db,
    fetch_employees_db,
    insert_employee_db,
    insert_patient_db,
    update_employee_db, // Make sure you import your patient DB functions here too!
                        // fetch_patients_db, delete_patient_db, insert_patient_db, update_patient_db
};
use crate::views::employees::{EmployeesMessage, FormField, add_employee_form};
use crate::views::patients::{PatientFormField, PatientsMessage, add_patient_form}; // Added Patient imports!

pub mod components;
pub mod database;
pub mod theme;
pub mod views;

#[derive(Debug, Clone, Default)]
pub struct DraftEmployee {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DraftPatient {
    pub name: String,
    pub birth_date: String,
    pub phone: String,
    pub address: String,
    pub show_picker: bool,
}

#[derive(Debug, Clone)]
pub enum ActiveModal {
    AddEmployee(DraftEmployee),
    EditEmployee {
        target_id: i32,
        draft: DraftEmployee,
    },
    // <-- ADDED PATIENT VARIANTS HERE -->
    AddPatient(DraftPatient),
    EditPatient {
        target_id: i32,
        draft: DraftPatient,
    },
}

#[derive(Debug, Clone)]
enum Message {
    Sidebar(sidebar::Message),
    EmployeeView(EmployeesMessage),
    PatientView(PatientsMessage),
}

pub struct ClinicApp {
    pub employees: Vec<Employee>,
    pub patients: Vec<Patient>, // <-- Added Patients vector!
    pub active_tab: Tab,
    pub active_modal: Option<ActiveModal>,
    pub is_saving: bool,
}

// Helper to grab the mutable patient draft
fn get_mut_patient_draft(modal: &mut Option<ActiveModal>) -> Option<&mut DraftPatient> {
    match modal {
        Some(ActiveModal::AddPatient(draft)) => Some(draft),
        Some(ActiveModal::EditPatient { draft, .. }) => Some(draft),
        _ => None,
    }
}

impl ClinicApp {
    fn new() -> Self {
        Self {
            employees: fetch_employees_db(),
            patients: vec![], // TODO: change to fetch_patients_db() when you make it!
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
                    let draft = match &mut self.active_modal {
                        Some(ActiveModal::AddEmployee(draft)) => draft,
                        Some(ActiveModal::EditEmployee { draft, .. }) => draft,
                        _ => return iced::Task::none(),
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

                EmployeesMessage::DeletedEmployee(result, deleted_id) => match result {
                    Ok(_) => {
                        self.employees.retain(|emp| emp.employee_id != deleted_id);
                    }
                    Err(e) => {
                        println!("Failed to delete employee: {}", e);
                    }
                },

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

                EmployeesMessage::SubmitForm => match &self.active_modal {
                    Some(ActiveModal::AddEmployee(draft)) => {
                        if draft.name.trim().is_empty() || draft.role.is_none() {
                            println!("Validation failed: Name and Role are required.");
                            return iced::Task::none();
                        }

                        let role = draft.role.clone().unwrap();
                        let name = draft.name.clone();
                        let phone = draft.phone.clone();
                        let email = draft.email.clone();

                        self.is_saving = true;

                        return iced::Task::perform(
                            insert_employee_db(role, name, phone, email),
                            |result| Message::EmployeeView(EmployeesMessage::EmployeeAdded(result)),
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

                    _ => return iced::Task::none(),
                },

                EmployeesMessage::EmployeeUpdated(result) => {
                    self.is_saving = false;
                    match result {
                        Ok(updated_employee) => {
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
                    self.is_saving = false;
                    match result {
                        Ok(new_employee) => {
                            self.employees.push(new_employee);
                            self.active_modal = None;
                        }
                        Err(e) => {
                            println!("Failed to add employee: {}", e);
                        }
                    }
                }
            },

            // =====================================
            // PATIENTS VIEW
            // =====================================
            Message::PatientView(patient_msg) => match patient_msg {
                PatientsMessage::OpenAddForm => {
                    self.active_modal = Some(ActiveModal::AddPatient(DraftPatient::default()));
                }

                PatientsMessage::OpenEditForm(patient) => {
                    self.active_modal = Some(ActiveModal::EditPatient {
                        target_id: patient.patient_id,
                        draft: DraftPatient {
                            name: patient.full_name,
                            birth_date: patient
                                .birth_date
                                .map(|d| d.format("%Y-%m-%d").to_string())
                                .unwrap_or_default(),
                            phone: patient.phone.unwrap_or_default(),
                            address: patient.address.unwrap_or_default(),
                            show_picker: false,
                        },
                    });
                }

                PatientsMessage::CloseAddForm => {
                    self.active_modal = None;
                }

                PatientsMessage::FieldChanged(field, new_value) => {
                    if let Some(draft) = get_mut_patient_draft(&mut self.active_modal) {
                        match field {
                            PatientFormField::Name => draft.name = new_value,
                            PatientFormField::BirthDate => draft.birth_date = new_value,
                            PatientFormField::Phone => draft.phone = new_value,
                            PatientFormField::Address => draft.address = new_value,
                        }
                    }
                }

                PatientsMessage::OpenDatePicker => {
                    if let Some(draft) = get_mut_patient_draft(&mut self.active_modal) {
                        draft.show_picker = true;
                    }
                }

                PatientsMessage::CancelDatePicker => {
                    if let Some(draft) = get_mut_patient_draft(&mut self.active_modal) {
                        draft.show_picker = false;
                    }
                }

                PatientsMessage::DateSelected(date) => {
                    if let Some(draft) = get_mut_patient_draft(&mut self.active_modal) {
                        // Format the iced_aw Date back into our string
                        draft.birth_date =
                            format!("{:04}-{:02}-{:02}", date.year, date.month, date.day);
                        draft.show_picker = false;
                    }
                }

                PatientsMessage::SubmitForm => {
                    match &self.active_modal {
                        Some(ActiveModal::AddPatient(draft)) => {
                            if draft.name.trim().is_empty() {
                                println!("Validation failed: Name is required");
                                return iced::Task::none();
                            }

                            let name = draft.name.clone();
                            let phone = draft.phone.clone();
                            let parse_birth_date =
                                chrono::NaiveDate::parse_from_str(&draft.birth_date, "%Y-%m-%d")
                                    .ok()
                                    .and_then(|d| d.and_hms_opt(0, 0, 0));
                            let address = draft.address.clone();

                            self.is_saving = true;

                            return iced::Task::perform(
                                insert_patient_db(name, phone, address, parse_birth_date),
                                |result| {
                                    Message::PatientView(PatientsMessage::PatientAdded(result))
                                },
                            );
                        }
                        Some(ActiveModal::EditPatient { target_id, draft }) => {
                            if draft.name.trim().is_empty() {
                                return iced::Task::none();
                            }

                            let id = *target_id;
                            let new_name = draft.name.clone();
                            let new_phone = draft.phone.clone();
                            let new_birth_date =
                                chrono::NaiveDate::parse_from_str(&draft.birth_date, "%Y-%m-%d")
                                    .ok()
                                    .and_then(|d| d.and_hms_opt(0, 0, 0));
                            let new_address = draft.address.clone();

                            self.is_saving = true;

                            return iced::Task::perform(
                                edit_patient_db(
                                    id,
                                    new_name,
                                    new_phone,
                                    new_address,
                                    new_birth_date,
                                ),
                                |result| {
                                    Message::PatientView(PatientsMessage::PatientUpdated(result))
                                },
                            );
                        } // TODO

                        _ => return iced::Task::none(),
                    }
                    // TODO: Paste the SubmitForm logic we wrote earlier here!
                    // (The one that parses chrono::NaiveDate and calls iced::Task::perform)
                }

                PatientsMessage::PatientAdded(result) => {
                    self.is_saving = false;
                    if let Ok(new_patient) = result {
                        self.patients.push(new_patient);
                        self.active_modal = None;
                    }
                }

                PatientsMessage::PatientUpdated(result) => {
                    self.is_saving = false;
                    if let Ok(updated_patient) = result {
                        if let Some(index) = self
                            .patients
                            .iter()
                            .position(|p| p.patient_id == updated_patient.patient_id)
                        {
                            self.patients[index] = updated_patient;
                        }
                        self.active_modal = None;
                    }
                }

                PatientsMessage::DeletePatient(id) => {
                    return iced::Task::perform(delete_patient_db(id), move |result| {
                        Message::PatientView(PatientsMessage::DeletedPatient(result, id))
                    });
                }

                PatientsMessage::DeletedPatient(result, deleted_id) => {
                    if result.is_ok() {
                        self.patients.retain(|p| p.patient_id != deleted_id);
                    }
                }
            },
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let sidebar_view = sidebar::view(&self.active_tab).map(Message::Sidebar);

        // 1. DYNAMIC MODAL ROUTING
        if let Some(modal) = &self.active_modal {
            match modal {
                // If it's an Employee Modal, draw the employee form
                ActiveModal::AddEmployee(d) | ActiveModal::EditEmployee { draft: d, .. } => {
                    let form = container(add_employee_form(
                        &d.name,
                        &d.phone,
                        &d.email,
                        d.role.as_deref(),
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill);
                    return Element::from(form).map(Message::EmployeeView);
                }

                // If it's a Patient Modal, draw the patient form
                ActiveModal::AddPatient(d) | ActiveModal::EditPatient { draft: d, .. } => {
                    let form = container(add_patient_form(
                        &d.name,
                        &d.birth_date,
                        &d.phone,
                        &d.address,
                        d.show_picker, // Pass the boolean!
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill);
                    return Element::from(form).map(Message::PatientView);
                }
            }
        }

        // 2. Render the main content area dynamically
        let content_view: Element<Message> = match self.active_tab {
            Tab::Dashboard => text("Dashboard View").size(30).into(),

            // Map the patient table!
            Tab::Patients => crate::views::patients::view(&self.patients).map(Message::PatientView),

            Tab::Employees => views::employees::view(&self.employees).map(Message::EmployeeView),
            Tab::Appointments => text("Appointments View").size(30).into(),
            Tab::Registry => text("Registry View").size(30).into(),
        };

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
