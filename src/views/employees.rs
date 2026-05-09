use crate::database::models::Employee;
use crate::database::operations::{
    delete_employee,
    fetch_all_employees_db,
    insert_doctor_db,
    insert_registry_worker_db,
    // Add an update_doctor_db / update_registry_db here later when you write them!
};
use crate::theme;
use iced::widget::{Space, button, column, container, pick_list, row, table, text, text_input};
use iced::{Alignment, Element, Font, Length};

const ROLE_OPTIONS: &[&str] = &["Doctor", "Registry", "General Staff"];

// ==========================================
// STATE & DRAFTS
// ==========================================
pub struct EmployeesTab {
    pub employees: Vec<Employee>,
    pub active_modal: Option<EmployeeModal>,
    pub is_saving: bool,
}

impl Default for EmployeesTab {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct DraftEmployee {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub role: Option<String>,
    // Subtype specific fields:
    pub specialty: String,
    pub office: String,
    pub window_number: String,
}

#[derive(Debug, Clone)]
pub enum EmployeeModal {
    Add(DraftEmployee),
    Edit {
        target_id: i32,
        draft: DraftEmployee,
    },
}

#[derive(Debug, Clone)]
pub enum EmployeeFormField {
    Name,
    Phone,
    Email,
    Role,
    Specialty,
    Office,
    WindowNumber,
}

#[derive(Debug, Clone)]
pub enum EmployeesMessage {
    OpenAddForm,
    OpenEditForm(Employee),
    CloseForm,
    FieldChanged(EmployeeFormField, String),
    SubmitForm,
    EmployeeAdded(Result<Employee, String>),
    EmployeeUpdated(Result<Employee, String>),
    DeleteEmployee(i32),
    DeletedEmployee(Result<usize, String>, i32),
}

// ==========================================
// COMPONENT LOGIC
// ==========================================
impl EmployeesTab {
    pub fn new() -> Self {
        Self {
            employees: fetch_all_employees_db(),
            active_modal: None,
            is_saving: false,
        }
    }

    fn get_mut_draft(&mut self) -> Option<&mut DraftEmployee> {
        match &mut self.active_modal {
            Some(EmployeeModal::Add(draft)) => Some(draft),
            Some(EmployeeModal::Edit { draft, .. }) => Some(draft),
            _ => None,
        }
    }

    pub fn update(&mut self, message: EmployeesMessage) -> iced::Task<EmployeesMessage> {
        match message {
            EmployeesMessage::OpenAddForm => {
                self.active_modal = Some(EmployeeModal::Add(DraftEmployee::default()));
                iced::Task::none()
            }
            EmployeesMessage::OpenEditForm(emp) => {
                self.active_modal = Some(EmployeeModal::Edit {
                    target_id: emp.employee_id,
                    draft: DraftEmployee {
                        name: emp.full_name,
                        phone: emp.phone.unwrap_or_default(),
                        email: emp.email.unwrap_or_default(),
                        role: Some(emp.role),
                        specialty: String::new(), // You'd fetch these joined details later for editing!
                        office: String::new(),
                        window_number: String::new(),
                    },
                });
                iced::Task::none()
            }
            EmployeesMessage::CloseForm => {
                self.active_modal = None;
                iced::Task::none()
            }
            EmployeesMessage::FieldChanged(field, new_value) => {
                if let Some(draft) = self.get_mut_draft() {
                    match field {
                        EmployeeFormField::Name => draft.name = new_value,
                        EmployeeFormField::Phone => draft.phone = new_value,
                        EmployeeFormField::Email => draft.email = new_value,
                        EmployeeFormField::Role => draft.role = Some(new_value),
                        EmployeeFormField::Specialty => draft.specialty = new_value,
                        EmployeeFormField::Office => draft.office = new_value,
                        EmployeeFormField::WindowNumber => draft.window_number = new_value,
                    }
                }
                iced::Task::none()
            }
            EmployeesMessage::SubmitForm => {
                match &self.active_modal {
                    Some(EmployeeModal::Add(draft)) => {
                        if draft.name.trim().is_empty() || draft.role.is_none() {
                            return iced::Task::none();
                        }

                        let role = draft.role.clone().unwrap();
                        self.is_saving = true;

                        // Route to the correct Database Transaction!
                        if role == "Doctor" {
                            iced::Task::perform(
                                insert_doctor_db(
                                    draft.name.clone(),
                                    draft.phone.clone(),
                                    draft.email.clone(),
                                    draft.specialty.clone(),
                                    draft.office.clone(),
                                ),
                                |res| match res {
                                    Ok((emp, _doc)) => EmployeesMessage::EmployeeAdded(Ok(emp)),
                                    Err(e) => EmployeesMessage::EmployeeAdded(Err(e)),
                                },
                            )
                        } else if role == "Registry" {
                            let window = draft.window_number.parse::<f64>().ok();
                            iced::Task::perform(
                                insert_registry_worker_db(
                                    draft.name.clone(),
                                    draft.phone.clone(),
                                    draft.email.clone(),
                                    window,
                                ),
                                |res| match res {
                                    Ok((emp, _reg)) => EmployeesMessage::EmployeeAdded(Ok(emp)),
                                    Err(e) => EmployeesMessage::EmployeeAdded(Err(e)),
                                },
                            )
                        } else {
                            // If it's general staff, you'd call a standard insert_employee_db here
                            println!("General staff insert not yet implemented!");
                            iced::Task::none()
                        }
                    }
                    Some(EmployeeModal::Edit { .. }) => {
                        // Implement update logic later
                        iced::Task::none()
                    }
                    _ => iced::Task::none(),
                }
            }
            EmployeesMessage::EmployeeAdded(result) => {
                self.is_saving = false;
                match result {
                    Ok(new_emp) => {
                        self.employees.push(new_emp);
                        self.active_modal = None;
                    }
                    Err(e) => println!("🚨 DB ERROR: {}", e),
                }
                iced::Task::none()
            }
            EmployeesMessage::EmployeeUpdated(result) => {
                self.is_saving = false;
                if let Ok(updated_emp) = result {
                    if let Some(idx) = self
                        .employees
                        .iter()
                        .position(|e| e.employee_id == updated_emp.employee_id)
                    {
                        self.employees[idx] = updated_emp;
                    }
                    self.active_modal = None;
                }
                iced::Task::none()
            }
            EmployeesMessage::DeleteEmployee(id) => {
                iced::Task::perform(delete_employee(id), move |res| {
                    EmployeesMessage::DeletedEmployee(res, id)
                })
            }
            EmployeesMessage::DeletedEmployee(result, deleted_id) => {
                if result.is_ok() {
                    self.employees.retain(|e| e.employee_id != deleted_id);
                }
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, EmployeesMessage> {
        if let Some(modal) = &self.active_modal {
            match modal {
                EmployeeModal::Add(draft) | EmployeeModal::Edit { draft, .. } => {
                    return container(add_employee_form(draft))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into();
                }
            }
        }

        let header = row![text("Staff Directory").color(theme::NAVY_SLATE).size(30.0)]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let add_btn = button(text("Add Staff Member"))
            .on_press(EmployeesMessage::OpenAddForm)
            .style(theme::primary_button)
            .padding([4, 8]);

        let action_bar = row![Space::new().width(Length::Fill), add_btn].width(Length::Fill);

        let content = column![
            header,
            Space::new().height(5.0),
            action_bar,
            Space::new().height(15.0),
            employees_table(&self.employees)
        ]
        .spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(theme::main_background)
            .into()
    }
}

// ==========================================
// UI HELPERS
// ==========================================
fn add_employee_form<'a>(draft: &'a DraftEmployee) -> Element<'a, EmployeesMessage> {
    let title = text("Staff Details").size(24);

    let name_input = text_input("Full Name", &draft.name)
        .on_input(|v| EmployeesMessage::FieldChanged(EmployeeFormField::Name, v))
        .padding(10);

    let phone_input = text_input("Phone Number", &draft.phone)
        .on_input(|v| EmployeesMessage::FieldChanged(EmployeeFormField::Phone, v))
        .padding(10);

    let email_input = text_input("Email", &draft.email)
        .on_input(|v| EmployeesMessage::FieldChanged(EmployeeFormField::Email, v))
        .padding(10);

    let selected_role = draft.role.as_deref();
    let role_dropdown = pick_list(ROLE_OPTIONS.to_vec(), selected_role, |s| {
        EmployeesMessage::FieldChanged(EmployeeFormField::Role, s.to_string())
    })
    .placeholder("Select Role")
    .width(Length::Fill)
    .padding(10);

    // Dynamic UI: Only show these inputs if Doctor or Registry is selected!
    let mut dynamic_fields = column![].spacing(10);

    if selected_role == Some("Doctor") {
        dynamic_fields = dynamic_fields
            .push(text("Specialty").size(14))
            .push(
                text_input("e.g., Cardiology", &draft.specialty)
                    .on_input(|v| EmployeesMessage::FieldChanged(EmployeeFormField::Specialty, v))
                    .padding(10),
            )
            .push(text("Office Number").size(14))
            .push(
                text_input("e.g., Room 102", &draft.office)
                    .on_input(|v| EmployeesMessage::FieldChanged(EmployeeFormField::Office, v))
                    .padding(10),
            );
    } else if selected_role == Some("Registry") {
        dynamic_fields = dynamic_fields.push(text("Window Number").size(14)).push(
            text_input("e.g., 1", &draft.window_number)
                .on_input(|v| EmployeesMessage::FieldChanged(EmployeeFormField::WindowNumber, v))
                .padding(10),
        );
    }

    let actions = row![
        button(text("Save").align_x(Alignment::Center))
            .on_press(EmployeesMessage::SubmitForm)
            .style(theme::primary_button)
            .padding([10, 20]),
        button(text("Cancel").align_x(Alignment::Center))
            .on_press(EmployeesMessage::CloseForm)
            .style(theme::secondary_button)
            .padding([10, 20])
    ]
    .spacing(15);

    let form_content = column![
        title,
        Space::new().height(20.0),
        text("Name").size(14),
        name_input,
        Space::new().height(10.0),
        text("Role").size(14),
        role_dropdown,
        Space::new().height(10.0),
        text("Phone").size(14),
        phone_input,
        Space::new().height(10.0),
        text("Email").size(14),
        email_input,
        Space::new().height(10.0),
        dynamic_fields,
        Space::new().height(20.0), // Inject dynamic fields here!
        actions
    ]
    .spacing(5)
    .max_width(500.0);

    container(form_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30)
        .style(theme::main_background)
        .into()
}

fn employees_table<'a>(employees: &'a [Employee]) -> Element<'a, EmployeesMessage> {
    let columns = vec![
        table::column(
            text("ID").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |e: &Employee| Element::from(text(e.employee_id.to_string())),
        )
        .width(Length::Fixed(50.0)),
        table::column(
            text("Name").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |e: &Employee| Element::from(text(e.full_name.clone())),
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Role").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |e: &Employee| Element::from(text(e.role.clone())),
        )
        .width(Length::Fixed(120.0)),
        table::column(
            text("Phone").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |e: &Employee| {
                Element::from(text(e.phone.clone().unwrap_or_else(|| "N/A".to_string())))
            },
        )
        .width(Length::Fixed(150.0)),
        table::column(
            text("Actions").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |e: &Employee| -> Element<'_, EmployeesMessage> {
                row![
                    button("Edit")
                        .style(theme::primary_button)
                        .on_press(EmployeesMessage::OpenEditForm(e.clone())),
                    button("Delete")
                        .style(theme::secondary_button)
                        .on_press(EmployeesMessage::DeleteEmployee(e.employee_id))
                ]
                .spacing(5)
                .into()
            },
        )
        .width(Length::Fixed(150.0)),
    ];

    let data_table = table(columns, employees).padding(10.0).separator_y(1.0);
    container(data_table)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::white_card)
        .into()
}
