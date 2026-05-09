use crate::components::shared::pagination::pagination;
use crate::components::shared::search_bar::search_bar;
use crate::database::models::{Appointment, Employee, Patient};
use crate::database::operations::{
    AppointmentPayload, delete_appointment_db, fetch_appointments_db, fetch_registry_joined_db,
    insert_appointment_db, update_appointment_db,
};
use crate::theme;

use iced::widget::{Space, button, column, container, pick_list, row, table, text, text_input};
use iced::{Alignment, Element, Font, Length};
use iced_aw::helpers::date_picker;

const STATUS_OPTIONS: &[&str] = &["Scheduled", "Completed", "Cancelled", "No Show"];
const ITEMS_PER_PAGE: usize = 14;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub id: i32,
    pub name: String,
}

impl std::fmt::Display for SelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// ==========================================
// STATE & DRAFTS
// ==========================================
pub struct AppointmentsTab {
    pub appointments: Vec<Appointment>,
    pub active_modal: Option<AppointmentModal>,
    pub is_saving: bool,

    // Search & Pagination State
    pub search_query: String,
    pub current_page: usize,
    pub page_data: Vec<Appointment>,
    pub total_pages: usize,

    // Filter State
    pub filter_status: Option<String>,
    pub filter_doctor: Option<i32>,
}

impl Default for AppointmentsTab {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct DraftAppointment {
    pub patient_id: Option<i32>,
    pub doctor_id: Option<i32>,
    pub date: String,
    pub time: String,
    pub status: String,
    pub reason: String,
    pub show_picker: bool,
}

#[derive(Debug, Clone)]
pub enum AppointmentModal {
    Add(DraftAppointment),
    Edit {
        target_id: i32,
        draft: DraftAppointment,
    },
}

#[derive(Debug, Clone)]
pub enum AppointmentFormField {
    Time,
    Status,
    Reason,
}

#[derive(Debug, Clone)]
pub enum AppointmentsMessage {
    OpenAddForm,
    OpenEditForm(Appointment),
    CloseForm,
    PatientSelected(i32),
    DoctorSelected(i32),
    FieldChanged(AppointmentFormField, String),
    OpenDatePicker,
    CancelDatePicker,
    DateSelected(iced_aw::core::date::Date),

    // Async
    SubmitForm,
    AppointmentAdded(Result<Appointment, String>),
    AppointmentUpdated(Result<Appointment, String>),
    DeleteAppointment(i32),
    DeletedAppointment(Result<usize, String>, i32),

    // Search bar & pagination
    SearchChanged(String),
    ClearSearch,
    NextPage,
    PreviousPage,

    // Filters
    StatusFilterChanged(String),
    DoctorFilterChanged(i32), // Using i32 here makes matching the ID much easier!
    ResetFilters,
}

// ==========================================
// COMPONENT LOGIC
// ==========================================
impl AppointmentsTab {
    #[must_use] 
    pub fn new() -> Self {
        let mut tab = Self {
            appointments: fetch_appointments_db(),
            active_modal: None,
            is_saving: false,
            search_query: String::new(),
            current_page: 1,
            page_data: Vec::new(),
            total_pages: 1,
            filter_status: None,
            filter_doctor: None,
        };
        tab.sync_view();
        tab
    }

    /// Calculates the current page data based on text search AND dropdown filters
    fn sync_view(&mut self) {
        let query = self.search_query.to_lowercase();

        let filtered: Vec<&Appointment> = self
            .appointments
            .iter()
            .filter(|a| {
                // 1. Text Search Filter (Checks Status, Reason, and Date)
                let search_matches = if query.is_empty() {
                    true
                } else {
                    let status = a.status.as_deref().unwrap_or("").to_lowercase();
                    let reason = a.reason.as_deref().unwrap_or("").to_lowercase();
                    let date = a
                        .appointment_date
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_default();

                    status.contains(&query) || reason.contains(&query) || date.contains(&query)
                };

                // 2. Dropdown Status Filter
                let status_matches = match &self.filter_status {
                    Some(s) if s == "All Statuses" => true,
                    Some(s) => a.status.as_deref() == Some(s.as_str()),
                    None => true,
                };

                // 3. Dropdown Doctor Filter
                let doctor_matches = match self.filter_doctor {
                    Some(-1) => true, // -1 represents "All Doctors"
                    Some(id) => a.doctor_id == id,
                    None => true,
                };

                // An appointment must pass ALL THREE filters to be shown!
                search_matches && status_matches && doctor_matches
            })
            .collect();

        // Calculate pages
        let total_items = filtered.len();
        self.total_pages = (total_items as f32 / ITEMS_PER_PAGE as f32).ceil() as usize;
        self.current_page = self.current_page.min(self.total_pages.max(1));

        let start_idx = (self.current_page.saturating_sub(1)) * ITEMS_PER_PAGE;
        let end_idx = (start_idx + ITEMS_PER_PAGE).min(total_items);

        // Slice for current page
        self.page_data = filtered[start_idx..end_idx]
            .iter()
            .map(|&a| a.clone())
            .collect();
    }

    fn get_mut_draft(&mut self) -> Option<&mut DraftAppointment> {
        match &mut self.active_modal {
            Some(AppointmentModal::Add(draft)) => Some(draft),
            Some(AppointmentModal::Edit { draft, .. }) => Some(draft),
            _ => None,
        }
    }

    pub fn update(&mut self, message: AppointmentsMessage) -> iced::Task<AppointmentsMessage> {
        match message {
            // --- FILTER MESSAGES ---
            AppointmentsMessage::StatusFilterChanged(s) => {
                self.filter_status = Some(s);
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            AppointmentsMessage::DoctorFilterChanged(id) => {
                self.filter_doctor = Some(id);
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            AppointmentsMessage::ResetFilters => {
                self.filter_status = None;
                self.filter_doctor = None;
                self.search_query.clear();
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }

            // --- SEARCH & PAGINATION MESSAGES ---
            AppointmentsMessage::SearchChanged(query) => {
                self.search_query = query;
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            AppointmentsMessage::ClearSearch => {
                self.search_query.clear();
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            AppointmentsMessage::NextPage => {
                self.current_page += 1;
                self.sync_view();
                iced::Task::none()
            }
            AppointmentsMessage::PreviousPage => {
                if self.current_page > 1 {
                    self.current_page -= 1;
                    self.sync_view();
                }
                iced::Task::none()
            }

            // --- FORM MESSAGES ---
            AppointmentsMessage::OpenAddForm => {
                let draft = DraftAppointment {
                    status: "Scheduled".to_string(),
                    ..Default::default()
                };
                self.active_modal = Some(AppointmentModal::Add(draft));
                iced::Task::none()
            }
            AppointmentsMessage::OpenEditForm(apt) => {
                self.active_modal = Some(AppointmentModal::Edit {
                    target_id: apt.appointment_id,
                    draft: DraftAppointment {
                        patient_id: Some(apt.patient_id),
                        doctor_id: Some(apt.doctor_id),
                        date: apt
                            .appointment_date
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_default(),
                        time: apt.appointment_time.unwrap_or_default(),
                        status: apt.status.unwrap_or_default(),
                        reason: apt.reason.unwrap_or_default(),
                        show_picker: false,
                    },
                });
                iced::Task::none()
            }
            AppointmentsMessage::CloseForm => {
                self.active_modal = None;
                iced::Task::none()
            }
            AppointmentsMessage::PatientSelected(id) => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.patient_id = Some(id);
                }
                iced::Task::none()
            }
            AppointmentsMessage::DoctorSelected(id) => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.doctor_id = Some(id);
                }
                iced::Task::none()
            }
            AppointmentsMessage::FieldChanged(field, new_value) => {
                if let Some(draft) = self.get_mut_draft() {
                    match field {
                        AppointmentFormField::Time => draft.time = new_value,
                        AppointmentFormField::Status => draft.status = new_value,
                        AppointmentFormField::Reason => draft.reason = new_value,
                    }
                }
                iced::Task::none()
            }
            AppointmentsMessage::OpenDatePicker => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.show_picker = true;
                }
                iced::Task::none()
            }
            AppointmentsMessage::CancelDatePicker => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.show_picker = false;
                }
                iced::Task::none()
            }
            AppointmentsMessage::DateSelected(date) => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.date = format!("{:04}-{:02}-{:02}", date.year, date.month, date.day);
                    draft.show_picker = false;
                }
                iced::Task::none()
            }
            AppointmentsMessage::SubmitForm => match &self.active_modal {
                Some(AppointmentModal::Add(draft)) => {
                    if draft.patient_id.is_none()
                        || draft.doctor_id.is_none()
                        || draft.date.is_empty()
                    {
                        return iced::Task::none();
                    }
                    let parsed_date = chrono::NaiveDate::parse_from_str(&draft.date, "%Y-%m-%d")
                        .ok()
                        .and_then(|d| d.and_hms_opt(0, 0, 0));
                    let payload_draft = draft.clone();
                    self.is_saving = true;

                    iced::Task::perform(
                        async move {
                            let registry_id = tokio::task::spawn_blocking(|| {
                                fetch_registry_joined_db()
                                    .first()
                                    .map_or(1, |(_, r)| r.employee_id)
                            })
                            .await
                            .unwrap_or(1);

                            let payload = AppointmentPayload {
                                patient_id: payload_draft.patient_id.unwrap(),
                                doctor_id: payload_draft.doctor_id.unwrap(),
                                registry_id,
                                date: parsed_date,
                                time: payload_draft.time,
                                status: payload_draft.status,
                                reason: payload_draft.reason,
                            };
                            insert_appointment_db(payload).await
                        },
                        AppointmentsMessage::AppointmentAdded,
                    )
                }
                Some(AppointmentModal::Edit { target_id, draft }) => {
                    if draft.patient_id.is_none()
                        || draft.doctor_id.is_none()
                        || draft.date.is_empty()
                    {
                        return iced::Task::none();
                    }

                    let parsed_date = chrono::NaiveDate::parse_from_str(&draft.date, "%Y-%m-%d")
                        .ok()
                        .and_then(|d| d.and_hms_opt(0, 0, 0));

                    let payload_draft = draft.clone();
                    let target = *target_id;
                    self.is_saving = true;

                    iced::Task::perform(
                        async move {
                            // We grab the registry ID exactly like we do for inserting
                            let registry_id = tokio::task::spawn_blocking(|| {
                                fetch_registry_joined_db()
                                    .first()
                                    .map_or(1, |(_, r)| r.employee_id)
                            })
                            .await
                            .unwrap_or(1);

                            let payload = AppointmentPayload {
                                patient_id: payload_draft.patient_id.unwrap(),
                                doctor_id: payload_draft.doctor_id.unwrap(),
                                registry_id,
                                date: parsed_date,
                                time: payload_draft.time,
                                status: payload_draft.status,
                                reason: payload_draft.reason,
                            };

                            // Call the update database function!
                            update_appointment_db(target, payload).await
                        },
                        AppointmentsMessage::AppointmentUpdated,
                    )
                }
                _ => iced::Task::none(),
            },
            AppointmentsMessage::AppointmentAdded(result) => {
                self.is_saving = false;
                if let Ok(new_apt) = result {
                    self.appointments.push(new_apt);
                    self.active_modal = None;
                    self.sync_view();
                }
                iced::Task::none()
            }
            AppointmentsMessage::AppointmentUpdated(result) => {
                self.is_saving = false;
                if let Ok(updated_apt) = result {
                    if let Some(idx) = self
                        .appointments
                        .iter()
                        .position(|a| a.appointment_id == updated_apt.appointment_id)
                    {
                        self.appointments[idx] = updated_apt;
                        self.sync_view();
                    }
                    self.active_modal = None;
                }
                iced::Task::none()
            }
            AppointmentsMessage::DeleteAppointment(id) => {
                iced::Task::perform(delete_appointment_db(id), move |res| {
                    AppointmentsMessage::DeletedAppointment(res, id)
                })
            }
            AppointmentsMessage::DeletedAppointment(result, deleted_id) => {
                if result.is_ok() {
                    self.appointments.retain(|a| a.appointment_id != deleted_id);
                    self.sync_view();
                }
                iced::Task::none()
            }
        }
    }

    pub fn view<'a>(
        &'a self,
        patients: &'a [Patient],
        employees: &'a [Employee],
    ) -> Element<'a, AppointmentsMessage> {
        if let Some(modal) = &self.active_modal {
            match modal {
                AppointmentModal::Add(draft) | AppointmentModal::Edit { draft, .. } => {
                    return container(add_appointment_form(draft, patients, employees))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into();
                }
            }
        }

        let header = row![text("Appointments").color(theme::NAVY_SLATE).size(30.0)]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        // 1. Prepare Filter Options
        let mut doctor_options = vec![SelectOption {
            id: -1,
            name: "All Doctors".to_string(),
        }];
        doctor_options.extend(employees.iter().filter(|e| e.role == "Doctor").map(|e| {
            SelectOption {
                id: e.employee_id,
                name: e.full_name.clone(),
            }
        }));

        let mut status_options = vec!["All Statuses".to_string()];
        status_options.extend(STATUS_OPTIONS.iter().map(std::string::ToString::to_string));

        // 2. Determine currently selected items
        let selected_doctor = doctor_options
            .iter()
            .find(|o| Some(o.id) == self.filter_doctor)
            .cloned();

        let selected_status = self.filter_status.clone();

        // 3. Build UI Controls
        let search_ui = search_bar(
            &self.search_query,
            "Search date, status, or reason...",
            AppointmentsMessage::SearchChanged,
            Some(AppointmentsMessage::ClearSearch),
        );

        let status_filter = pick_list(
            status_options,
            selected_status,
            AppointmentsMessage::StatusFilterChanged,
        )
        .placeholder("Filter Status")
        .width(Length::Fixed(150.0))
        .padding(8);

        let doctor_filter = pick_list(doctor_options, selected_doctor, |opt| {
            AppointmentsMessage::DoctorFilterChanged(opt.id)
        })
        .placeholder("Filter Doctor")
        .width(Length::Fixed(200.0))
        .padding(8);

        let reset_btn = button(text("Reset"))
            .on_press(AppointmentsMessage::ResetFilters)
            .style(theme::secondary_button)
            .padding([8, 12]);

        let add_btn = button(text("Book Appointment"))
            .on_press(AppointmentsMessage::OpenAddForm)
            .style(theme::primary_button)
            .padding([8, 12]);

        // Stack the search and filters next to each other
        let action_bar = row![
            search_ui,
            status_filter,
            doctor_filter,
            reset_btn,
            Space::new().width(Length::Fill),
            add_btn
        ]
        .align_y(Alignment::Center)
        .spacing(15)
        .width(Length::Fill);

        // 4. Build Pagination and Footer
        let pagination_ui = pagination(
            self.current_page,
            self.total_pages,
            AppointmentsMessage::PreviousPage,
            AppointmentsMessage::NextPage,
        );

        let footer_text = text(format!("Total Appointments: {}", self.appointments.len()))
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .size(16);

        let bottom_bar = row![footer_text, Space::new().width(Length::Fill), pagination_ui]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        // 5. Assemble layout
        let content = column![
            header,
            Space::new().height(15.0),
            action_bar,
            Space::new().height(15.0),
            appointments_table(&self.page_data, patients, employees),
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
fn add_appointment_form<'a>(
    draft: &'a DraftAppointment,
    patients: &'a [Patient],
    employees: &'a [Employee],
) -> Element<'a, AppointmentsMessage> {
    let title = text("Book Appointment").size(24);

    let patient_options: Vec<SelectOption> = patients
        .iter()
        .map(|p| SelectOption {
            id: p.patient_id,
            name: p.full_name.clone(),
        })
        .collect();

    let doctor_options: Vec<SelectOption> = employees
        .iter()
        .filter(|e| e.role == "Doctor")
        .map(|e| SelectOption {
            id: e.employee_id,
            name: e.full_name.clone(),
        })
        .collect();

    let selected_patient = draft
        .patient_id
        .and_then(|id| patient_options.iter().find(|o| o.id == id).cloned());
    let selected_doctor = draft
        .doctor_id
        .and_then(|id| doctor_options.iter().find(|o| o.id == id).cloned());
    let selected_status = STATUS_OPTIONS.iter().find(|&&s| s == draft.status).copied();

    let patient_dropdown = pick_list(patient_options, selected_patient, |opt| {
        AppointmentsMessage::PatientSelected(opt.id)
    })
    .placeholder("Select Patient")
    .width(Length::Fill)
    .padding(10);

    let doctor_dropdown = pick_list(doctor_options, selected_doctor, |opt| {
        AppointmentsMessage::DoctorSelected(opt.id)
    })
    .placeholder("Select Doctor")
    .width(Length::Fill)
    .padding(10);

    let date_btn = button(text(if draft.date.is_empty() {
        "Select Date"
    } else {
        &draft.date
    }))
    .on_press(AppointmentsMessage::OpenDatePicker)
    .padding(10)
    .width(Length::Fill)
    .style(theme::secondary_button);

    let time_input = text_input("HH:MM", &draft.time)
        .on_input(|v| AppointmentsMessage::FieldChanged(AppointmentFormField::Time, v))
        .padding(10);

    let status_dropdown = pick_list(STATUS_OPTIONS.to_vec(), selected_status, |s| {
        AppointmentsMessage::FieldChanged(AppointmentFormField::Status, s.to_string())
    })
    .placeholder("Select Status")
    .width(Length::Fill)
    .padding(10);

    let reason_input = text_input("Reason for visit", &draft.reason)
        .on_input(|v| AppointmentsMessage::FieldChanged(AppointmentFormField::Reason, v))
        .padding(10);

    let actions = row![
        button(text("Save").align_x(Alignment::Center))
            .on_press(AppointmentsMessage::SubmitForm)
            .style(theme::primary_button)
            .padding([10, 20]),
        button(text("Cancel").align_x(Alignment::Center))
            .on_press(AppointmentsMessage::CloseForm)
            .style(theme::secondary_button)
            .padding([10, 20])
    ]
    .spacing(15);

    let form_content = column![
        title,
        Space::new().height(20.0),
        text("Patient").size(14),
        patient_dropdown,
        Space::new().height(10.0),
        text("Doctor").size(14),
        doctor_dropdown,
        Space::new().height(10.0),
        text("Date (YYYY-MM-DD)").size(14),
        date_btn,
        Space::new().height(10.0),
        text("Time").size(14),
        time_input,
        Space::new().height(10.0),
        text("Status").size(14),
        status_dropdown,
        Space::new().height(10.0),
        text("Reason").size(14),
        reason_input,
        Space::new().height(20.0),
        actions
    ]
    .spacing(5)
    .max_width(500.0);

    let form_container = container(form_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30)
        .style(theme::main_background);

    let initial_date = chrono::NaiveDate::parse_from_str(&draft.date, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::Local::now().date_naive());

    date_picker(
        draft.show_picker,
        initial_date,
        form_container,
        AppointmentsMessage::CancelDatePicker,
        AppointmentsMessage::DateSelected,
    )
    .into()
}

fn appointments_table<'a>(
    appointments: &'a [Appointment],
    patients: &'a [Patient],
    employees: &'a [Employee],
) -> Element<'a, AppointmentsMessage> {
    let columns = vec![
        table::column(
            text("Date").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |a: &Appointment| {
                Element::from(text(
                    a.appointment_date.map_or_else(|| "N/A".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                ))
            },
        )
        .width(Length::Fixed(120.0)),
        table::column(
            text("Patient").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |a: &Appointment| {
                Element::from(text(
                    patients
                        .iter()
                        .find(|p| p.patient_id == a.patient_id).map_or_else(|| "Unknown".to_string(), |p| p.full_name.clone()),
                ))
            },
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Doctor").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |a: &Appointment| {
                Element::from(text(
                    employees
                        .iter()
                        .find(|e| e.employee_id == a.doctor_id).map_or_else(|| "Unknown".to_string(), |e| e.full_name.clone()),
                ))
            },
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Status").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |a: &Appointment| {
                Element::from(text(a.status.clone().unwrap_or_else(|| "N/A".to_string())))
            },
        )
        .width(Length::Fixed(100.0)),
        table::column(
            text("Reason").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |a: &Appointment| {
                Element::from(text(a.reason.clone().unwrap_or_else(|| "N/A".to_string())))
            },
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Actions").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |a: &Appointment| -> Element<'_, AppointmentsMessage> {
                row![
                    button("Edit")
                        .style(theme::primary_button)
                        .on_press(AppointmentsMessage::OpenEditForm(a.clone())),
                    button("Delete")
                        .style(theme::secondary_button)
                        .on_press(AppointmentsMessage::DeleteAppointment(a.appointment_id))
                ]
                .spacing(5)
                .into()
            },
        )
        .width(Length::Fixed(150.0)),
    ];

    let data_table = table(columns, appointments).padding(10.0).separator_y(1.0);

    container(data_table)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::white_card)
        .into()
}
