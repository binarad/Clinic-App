use crate::database::models::{Appointment, Employee, Patient};
use crate::theme;
use iced::widget::{Space, button, column, container, pick_list, row, table, text, text_input};
use iced::{Alignment, Element, Font, Length};
use iced_aw::helpers::date_picker;

// Helper struct for our Dropdowns (Shows name, saves ID)
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

#[derive(Debug, Clone)]
pub enum AppointmentFormField {
    Time,
    Status,
    Reason,
}

#[derive(Debug, Clone, Default)]
pub struct DraftAppointment {
    // TODO Update struct
    pub patient_id: Option<i32>,
    pub doctor_id: Option<i32>, // The doctor/staff member
    pub registry_id: i32,
    pub date: String, // We can use iced_aw date_picker here too!
    pub time: String, // e.g., "14:30"
    pub status: String,
    pub reason: String,
    pub show_picker: bool,
}

impl DraftAppointment {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            doctor_id: None,
            registry_id: 1,
            date: String::new(),
            time: String::new(),
            status: String::from("Scheduled"),
            reason: String::new(),
            show_picker: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppointmentsMessage {
    OpenAddForm,
    OpenEditForm(Appointment),
    CloseAddForm,

    // Form inputs
    PatientSelected(i32),
    DoctorSelected(i32),
    FieldChanged(AppointmentFormField, String),

    // Date Picker
    OpenDatePicker,
    CancelDatePicker,
    DateSelected(iced_aw::core::date::Date),

    // Async Actions
    SubmitForm,
    DeleteAppointment(i32),
    AppointmentAdded(Result<Appointment, String>),
    AppointmentUpdated(Result<Appointment, String>),
    DeletedAppointment(Result<usize, String>, i32),
}

const STATUS_OPTIONS: &[&str] = &["Scheduled", "Completed", "Cancelled", "No Show"];

pub fn view<'a>(
    appointments: &'a [Appointment],
    patients: &'a [Patient],
    employees: &'a [Employee],
) -> Element<'a, AppointmentsMessage> {
    let header = row![text("Appointments").color(theme::NAVY_SLATE).size(30.0)]
        .align_y(Alignment::Center)
        .width(Length::Fill);

    let add_btn = button(text("Book Appointment"))
        .on_press(AppointmentsMessage::OpenAddForm)
        .style(theme::primary_button)
        .padding([4, 8]);

    let action_bar = row![Space::new().width(Length::Fill), add_btn].width(Length::Fill);

    let table_content = appointments_table(appointments, patients, employees);

    let content = column![
        header,
        Space::new().height(5.0),
        action_bar,
        Space::new().height(15.0),
        table_content,
    ]
    .spacing(0);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(theme::main_background)
        .into()
}

// =============
// DATA TABLE
// =============
pub fn appointments_table<'a>(
    appointments: &'a [Appointment],
    patients: &'a [Patient],
    employees: &'a [Employee],
) -> Element<'a, AppointmentsMessage> {
    let columns = vec![
        table::column(
            text("Date/Time").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |apt: &Appointment| {
                let date_str = apt
                    .appointment_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                let time_str = apt.appointment_time.as_deref().unwrap_or("N/A");
                Element::from(text(format!("{} {}", date_str, time_str)))
            },
        )
        .width(Length::Fixed(150.0)),
        table::column(
            text("Patient").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |apt: &Appointment| {
                let patient_name = patients
                    .iter()
                    .find(|p| p.patient_id == apt.patient_id)
                    .map(|p| p.full_name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                Element::from(text(patient_name))
            },
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Doctor").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |apt: &Appointment| {
                let doc_name = employees
                    .iter()
                    .find(|e| e.employee_id == apt.doctor_id)
                    .map(|e| e.full_name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                Element::from(text(doc_name))
            },
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Status").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |apt: &Appointment| Element::from(text(apt.status.as_deref().unwrap_or("N/A"))),
        )
        .width(Length::Fixed(100.0)),
        table::column(
            text("Reason").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |apt: &Appointment| {
                Element::from(
                    text(apt.reason.as_deref().unwrap_or("N/A"))
                        .wrapping(text::Wrapping::WordOrGlyph),
                )
            },
        )
        .width(Length::FillPortion(3)),
        table::column(
            text("Actions").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |apt: &Appointment| -> Element<'_, AppointmentsMessage> {
                let edit_btn = button("Edit")
                    .style(theme::primary_button)
                    .on_press(AppointmentsMessage::OpenEditForm(apt.clone()));
                let delete_btn = button("Delete")
                    .style(theme::secondary_button)
                    .on_press(AppointmentsMessage::DeleteAppointment(apt.appointment_id));

                row![edit_btn, delete_btn].spacing(5).into()
            },
        )
        .width(Length::Fixed(150.0)),
    ];

    let data_table = table(columns, appointments).padding(10.0).separator_y(1.0);

    let footer = container(
        text(format!("Total Appointments: {}", appointments.len())).font(Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }),
    )
    .width(Length::Fill)
    .align_x(Alignment::Start)
    .padding([10, 20]);

    container(column![
        data_table,
        Space::new().height(Length::Fill),
        footer
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .style(theme::white_card)
    .into()
}

// =================
// ADD APPOINTMENT FORM
// =================
pub fn add_appointment_form<'a>(
    draft: &'a DraftAppointment,
    patients: &'a [Patient],
    employees: &'a [Employee],
) -> Element<'a, AppointmentsMessage> {
    let title = text("Book Appointment").size(24);

    // 1. Map data into SelectOptions for the Dropdowns
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

    // 2. Find the currently selected options based on the draft IDs
    let selected_patient = draft
        .patient_id
        .and_then(|id| patient_options.iter().find(|o| o.id == id).cloned());
    let selected_doctor = draft
        .doctor_id
        .and_then(|id| doctor_options.iter().find(|o| o.id == id).cloned());
    let selected_status = STATUS_OPTIONS.iter().find(|&&s| s == draft.status).copied();

    // 3. Build the UI Inputs
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

    let time_input = text_input("HH:MM (e.g., 14:30)", &draft.time)
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
            .on_press(AppointmentsMessage::CloseAddForm)
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
