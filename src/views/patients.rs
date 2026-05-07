use crate::database::models::Patient;
use crate::theme;
use iced::widget::{Space, button, column, container, row, table, text, text_input};
use iced::{Alignment, Element, Font, Length};
use iced_aw::helpers::date_picker; // <-- Import the helper!

#[derive(Debug, Clone)]
pub enum PatientFormField {
    Name,
    Phone,
    Address,
    BirthDate,
}

#[derive(Debug, Clone)]
pub enum PatientsMessage {
    OpenAddForm,
    OpenEditForm(Patient),
    CloseAddForm,

    OpenDatePicker,
    CancelDatePicker,

    // Form inputs
    FieldChanged(PatientFormField, String),
    DateSelected(iced_aw::core::date::Date),

    // Async Actions
    SubmitForm,
    DeletePatient(i32),
    PatientAdded(Result<Patient, String>),
    PatientUpdated(Result<Patient, String>),
    DeletedPatient(Result<usize, String>, i32),
}

pub fn view<'a>(patients: &'a [Patient]) -> Element<'a, PatientsMessage> {
    let header = row![text("Patients").color(theme::NAVY_SLATE).size(30.0)]
        .align_y(Alignment::Center)
        .width(Length::Fill);

    let add_btn = button(text("Add New Patient"))
        .on_press(PatientsMessage::OpenAddForm)
        .style(theme::primary_button)
        .padding([4, 8]);

    let action_bar = row![Space::new().width(Length::Fill), add_btn,].width(Length::Fill);

    let table_content = patients_table(patients);

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
pub fn patients_table<'a>(patients: &'a [Patient]) -> Element<'a, PatientsMessage> {
    let columns = vec![
        table::column(
            text("ID").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |patient: &Patient| Element::from(text(patient.patient_id.to_string())),
        )
        .width(Length::Fixed(50.0)),
        table::column(
            text("Full Name").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |patient: &Patient| Element::from(text(&patient.full_name)),
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Birth Date").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |patient: &Patient| {
                let date_str = patient
                    .birth_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                Element::from(text(date_str))
            },
        )
        .width(Length::Fixed(120.0)),
        table::column(
            text("Phone").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |patient: &Patient| {
                let phone_text = patient.phone.as_deref().unwrap_or("N/A");
                Element::from(text(phone_text))
            },
        )
        .width(Length::Fixed(120.0)),
        table::column(
            text("Address").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |patient: &Patient| {
                let address_text = patient.address.as_deref().unwrap_or("N/A");
                Element::from(
                    text(address_text)
                        .width(Length::Fill)
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
            |patient: &Patient| -> Element<'_, PatientsMessage> {
                let edit_btn = button("Edit")
                    .style(theme::primary_button)
                    .on_press(PatientsMessage::OpenEditForm(patient.clone()));
                let delete_btn = button("Delete")
                    .style(theme::secondary_button)
                    .on_press(PatientsMessage::DeletePatient(patient.patient_id));

                row![edit_btn, delete_btn].spacing(5).into()
            },
        )
        .width(Length::Fixed(150.0)),
    ];

    let data_table = table(columns, patients)
        .padding(10.0)
        .separator_y(1.0)
        .separator_x(0.0);

    let total_count = patients.len();
    let footer = container(text(format!("Total Patients: {}", total_count)).font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    }))
    .width(Length::Fill)
    .align_x(Alignment::Start)
    .padding([10, 20]);

    let content = column![data_table, Space::new().height(Length::Fill), footer];

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::white_card)
        .into()
}

// =================
// ADD PATIENT FORM
// =================

pub fn add_patient_form<'a>(
    draft_name: &'a str,
    draft_birth_date: &'a str,
    draft_phone: &'a str,
    draft_address: &'a str,
    show_picker: bool, // <-- Added this!
) -> Element<'a, PatientsMessage> {
    let title = text("Add New Patient").size(24);

    let name_input = text_input("Full Name", draft_name)
        .on_input(|value| PatientsMessage::FieldChanged(PatientFormField::Name, value))
        .padding(10);

    // 1. The Date Picker Button
    let dob_display_text = if draft_birth_date.is_empty() {
        "Click to Select Date"
    } else {
        draft_birth_date
    };

    let dob_btn = button(text(dob_display_text))
        .on_press(PatientsMessage::OpenDatePicker)
        .padding(10)
        .width(Length::Fill)
        .style(theme::secondary_button);

    let phone_input = text_input("Phone Number", draft_phone)
        .on_input(|value| PatientsMessage::FieldChanged(PatientFormField::Phone, value))
        .padding(10);

    let address_input = text_input("Home Address", draft_address)
        .on_input(|value| PatientsMessage::FieldChanged(PatientFormField::Address, value))
        .padding(10);

    let save_btn = button(text("Save Patient").align_x(Alignment::Center))
        .on_press(PatientsMessage::SubmitForm)
        .style(theme::primary_button)
        .padding([10, 20]);

    let cancel_btn = button(text("Cancel").align_x(Alignment::Center))
        .on_press(PatientsMessage::CloseAddForm)
        .style(theme::secondary_button)
        .padding([10, 20]);

    let actions = row![save_btn, cancel_btn].spacing(15);

    let form_content = column![
        title,
        Space::new().height(20.0),
        text("Full Name").size(14),
        name_input,
        Space::new().height(10.0),
        text("Birth Date (YYYY-MM-DD)").size(14),
        dob_btn, // <-- Replaced the text input with the button!
        Space::new().height(10.0),
        text("Phone").size(14),
        phone_input,
        Space::new().height(10.0),
        text("Address").size(14),
        address_input,
        Space::new().height(10.0),
        actions
    ]
    .spacing(5)
    .max_width(500.0);

    let form_container = container(form_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30)
        .style(theme::main_background);

    let initial_date = chrono::NaiveDate::parse_from_str(draft_birth_date, "%Y-%m-%d")
        // If it's empty/invalid, open the calendar to Jan 1, 2000 (Great UX for birth dates!)
        .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    // 2. Wrap the form in the date_picker overlay
    date_picker(
        show_picker,
        initial_date,
        form_container,
        PatientsMessage::CancelDatePicker,
        PatientsMessage::DateSelected,
    )
    .into()
}
