use crate::components::shared::pagination::pagination;
use crate::components::shared::search_bar::search_bar;
use crate::database::models::Employee;
use crate::database::operations::{
    delete_employee, fetch_all_employees_db, insert_doctor_db, insert_employee_db,
    insert_registry_worker_db, update_employee_db,
};
use crate::theme;

use iced::widget::{Space, button, column, container, pick_list, row, table, text, text_input};
use iced::{Alignment, Element, Font, Length};

const ROLE_OPTIONS: &[&str] = &["Doctor", "Registry", "General Staff"];
const ITEMS_PER_PAGE: usize = 14;

// ==========================================
// STATE & DRAFTS
// ==========================================
pub struct EmployeesTab {
    pub employees: Vec<Employee>,
    pub active_modal: Option<EmployeeModal>,
    pub is_saving: bool,
    pub search_query: String,
    pub current_page: usize,
    pub page_data: Vec<Employee>,
    pub total_pages: usize,
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
    SearchChanged(String),
    ClearSearch,
    NextPage,
    PreviousPage,
}

// ==========================================
// COMPONENT LOGIC
// ==========================================
impl EmployeesTab {
    #[must_use] 
    pub fn new() -> Self {
        let mut tab = Self {
            employees: fetch_all_employees_db(),
            active_modal: None,
            is_saving: false,
            search_query: String::new(),
            current_page: 1,
            page_data: Vec::new(),
            total_pages: 1,
        };
        tab.sync_view();
        tab
    }

    fn sync_view(&mut self) {
        let query = self.search_query.to_lowercase();

        let filtered: Vec<&Employee> = self
            .employees
            .iter()
            .filter(|e| {
                if query.is_empty() {
                    return true;
                }
                let name_matches = e.full_name.to_lowercase().contains(&query);
                let role_matches = e.role.to_lowercase().contains(&query);
                let phone_matches = e.phone.as_deref().unwrap_or("").contains(&query);
                let email_matches = e
                    .email
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&query);
                name_matches || role_matches || phone_matches || email_matches
            })
            .collect();

        let total_items = filtered.len();
        self.total_pages = (total_items as f32 / ITEMS_PER_PAGE as f32).ceil() as usize;
        self.current_page = self.current_page.min(self.total_pages.max(1));

        let start_idx = (self.current_page.saturating_sub(1)) * ITEMS_PER_PAGE;
        let end_idx = (start_idx + ITEMS_PER_PAGE).min(total_items);

        self.page_data = filtered[start_idx..end_idx]
            .iter()
            .map(|&e| e.clone())
            .collect();
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
            EmployeesMessage::SearchChanged(query) => {
                self.search_query = query;
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            EmployeesMessage::ClearSearch => {
                self.search_query.clear();
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            EmployeesMessage::NextPage => {
                self.current_page += 1;
                self.sync_view();
                iced::Task::none()
            }
            EmployeesMessage::PreviousPage => {
                if self.current_page > 1 {
                    self.current_page -= 1;
                    self.sync_view();
                }
                iced::Task::none()
            }

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
                        specialty: String::new(),
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
            EmployeesMessage::SubmitForm => match &self.active_modal {
                Some(EmployeeModal::Add(draft)) => {
                    if draft.name.trim().is_empty() || draft.role.is_none() {
                        return iced::Task::none();
                    }
                    let role = draft.role.clone().unwrap();
                    self.is_saving = true;

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
                        iced::Task::perform(
                            insert_employee_db(
                                role,
                                draft.name.clone(),
                                draft.phone.clone(),
                                draft.email.clone(),
                            ),
                            EmployeesMessage::EmployeeAdded,
                        )
                    }
                }
                Some(EmployeeModal::Edit { target_id, draft }) => {
                    if draft.name.trim().is_empty() || draft.role.is_none() {
                        return iced::Task::none();
                    }

                    let role = draft.role.clone().unwrap();
                    let id = *target_id;
                    let name = draft.name.clone();
                    let phone = draft.phone.clone();
                    let email = draft.email.clone();

                    self.is_saving = true;

                    // Note: Your update_employee_db safely handles the base table.
                    // Updating Subtypes (Doctor/Registry) is incredibly complex in SQL,
                    // so we are just sticking to updating the base contact info here!
                    iced::Task::perform(update_employee_db(id, role, name, phone, email), |res| {
                        EmployeesMessage::EmployeeUpdated(res)
                    })
                }
                _ => iced::Task::none(),
            },
            EmployeesMessage::EmployeeAdded(result) => {
                self.is_saving = false;
                if let Ok(new_emp) = result {
                    self.employees.push(new_emp);
                    self.active_modal = None;
                    self.sync_view();
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
                        self.sync_view();
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
                    self.sync_view();
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

        let search_ui = search_bar(
            &self.search_query,
            "Search name, role, email, or phone...",
            EmployeesMessage::SearchChanged,
            Some(EmployeesMessage::ClearSearch),
        );

        let pagination_ui = pagination(
            self.current_page,
            self.total_pages,
            EmployeesMessage::PreviousPage,
            EmployeesMessage::NextPage,
        );

        let add_btn = button(text("Add Staff Member"))
            .on_press(EmployeesMessage::OpenAddForm)
            .style(theme::primary_button)
            .padding([8, 12]);

        let action_bar = row![search_ui, Space::new().width(Length::Fill), add_btn]
            .align_y(Alignment::Center)
            .spacing(15)
            .width(Length::Fill);

        let footer_text = text(format!("Total Staff: {}", self.employees.len()))
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .size(16);

        let bottom_bar = row![footer_text, Space::new().width(Length::Fill), pagination_ui]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let content = column![
            header,
            Space::new().height(15.0),
            action_bar,
            Space::new().height(15.0),
            employees_table(&self.page_data),
            Space::new().height(15.0),
            bottom_bar
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
fn add_employee_form(draft: &DraftEmployee) -> Element<'_, EmployeesMessage> {
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
        Space::new().height(20.0),
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

fn employees_table(employees: &[Employee]) -> Element<'_, EmployeesMessage> {
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
            text("Email").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |e: &Employee| {
                Element::from(text(e.email.clone().unwrap_or_else(|| "N/A".to_string())))
            },
        )
        .width(Length::FillPortion(2)),
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
