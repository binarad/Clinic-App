use crate::database::models::Patient;
use crate::database::operations::{
    delete_patient_db, edit_patient_db, fetch_patient_db, insert_patient_db,
};
use crate::theme;
use iced::widget::{Space, button, column, container, row, table, text, text_input};
use iced::{Alignment, Element, Font, Length};
use iced_aw::helpers::date_picker; // <-- Import the helper!

// =================================
// STATE & DRAFTS
// =================================
pub struct PatientsTab {
    pub patients: Vec<Patient>,
    pub active_modal: Option<PatientModal>,
    pub is_saving: bool,
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
pub enum PatientModal {
    Add(DraftPatient),
    Edit { target_id: i32, draft: DraftPatient },
}

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
    CloseForm,

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

impl Default for PatientsTab {
    fn default() -> Self {
        Self::new()
    }
}
impl PatientsTab {
    pub fn new() -> Self {
        Self {
            patients: fetch_patient_db(),
            active_modal: None,
            is_saving: false,
        }
    }

    // Helper to grab the mutable patient draft
    fn get_mut_draft(&mut self) -> Option<&mut DraftPatient> {
        match &mut self.active_modal {
            Some(PatientModal::Add(draft)) => Some(draft),
            Some(PatientModal::Edit { draft, .. }) => Some(draft),
            _ => None,
        }
    }

    pub fn update(&mut self, message: PatientsMessage) -> iced::Task<PatientsMessage> {
        match message {
            PatientsMessage::OpenAddForm => {
                self.active_modal = Some(PatientModal::Add(DraftPatient::default()));
                iced::Task::none()
            }

            PatientsMessage::OpenEditForm(patient) => {
                self.active_modal = Some(PatientModal::Edit {
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
                iced::Task::none()
            }

            PatientsMessage::CloseForm => {
                self.active_modal = None;
                iced::Task::none()
            }

            PatientsMessage::FieldChanged(field, new_value) => {
                if let Some(draft) = self.get_mut_draft() {
                    match field {
                        PatientFormField::Name => draft.name = new_value,
                        PatientFormField::BirthDate => draft.birth_date = new_value,
                        PatientFormField::Phone => draft.phone = new_value,
                        PatientFormField::Address => draft.address = new_value,
                    }
                }
                iced::Task::none()
            }

            PatientsMessage::OpenDatePicker => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.show_picker = true;
                }
                iced::Task::none()
            }

            PatientsMessage::CancelDatePicker => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.show_picker = false;
                }
                iced::Task::none()
            }

            PatientsMessage::DateSelected(date) => {
                if let Some(draft) = self.get_mut_draft() {
                    draft.birth_date =
                        format!("{:04}-{:02}-{:02}", date.year, date.month, date.day);
                    draft.show_picker = false;
                }
                iced::Task::none()
            }

            PatientsMessage::SubmitForm => match &self.active_modal {
                Some(PatientModal::Add(draft)) => {
                    if draft.name.trim().is_empty() {
                        return iced::Task::none();
                    }

                    let name = draft.name.clone();
                    let phone = draft.phone.clone();
                    let address = draft.address.clone();
                    let parsed_date =
                        chrono::NaiveDate::parse_from_str(&draft.birth_date, "%Y-%m-%d")
                            .ok()
                            .and_then(|d| d.and_hms_opt(0, 0, 0));

                    self.is_saving = true;
                    iced::Task::perform(
                        insert_patient_db(name, phone, address, parsed_date),
                        PatientsMessage::PatientAdded,
                    )
                }
                Some(PatientModal::Edit { target_id, draft }) => {
                    if draft.name.trim().is_empty() {
                        return iced::Task::none();
                    }

                    let id = *target_id;
                    let name = draft.name.clone();
                    let phone = draft.phone.clone();
                    let address = draft.address.clone();
                    let parsed_date =
                        chrono::NaiveDate::parse_from_str(&draft.birth_date, "%Y-%m-%d")
                            .ok()
                            .and_then(|d| d.and_hms_opt(0, 0, 0));

                    self.is_saving = true;
                    iced::Task::perform(
                        edit_patient_db(id, name, phone, address, parsed_date),
                        PatientsMessage::PatientUpdated,
                    )
                }
                _ => iced::Task::none(),
            },
            PatientsMessage::PatientAdded(result) => {
                self.is_saving = false;
                if let Ok(new_patient) = result {
                    self.patients.push(new_patient);
                    self.active_modal = None;
                }
                iced::Task::none()
            }
            PatientsMessage::PatientUpdated(result) => {
                self.is_saving = false;
                if let Ok(updated_patient) = result {
                    if let Some(idx) = self
                        .patients
                        .iter()
                        .position(|p| p.patient_id == updated_patient.patient_id)
                    {
                        self.patients[idx] = updated_patient;
                    }
                    self.active_modal = None;
                }
                iced::Task::none()
            }
            PatientsMessage::DeletePatient(id) => {
                iced::Task::perform(delete_patient_db(id), move |res| {
                    PatientsMessage::DeletedPatient(res, id)
                })
            }
            PatientsMessage::DeletedPatient(result, deleted_id) => {
                if result.is_ok() {
                    self.patients.retain(|p| p.patient_id != deleted_id);
                }
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, PatientsMessage> {
        // If a modal is active, render it
        if let Some(modal) = &self.active_modal {
            match modal {
                PatientModal::Add(draft) | PatientModal::Edit { draft, .. } => {
                    return container(add_patient_form(draft))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into();
                }
            }
        }

        let header = row![text("Patients").color(theme::NAVY_SLATE).size(30.0)]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let add_btn = button(text("Add New Patient"))
            .on_press(PatientsMessage::OpenAddForm)
            .style(theme::primary_button)
            .padding([4, 8]);

        let action_bar = row![Space::new().width(Length::Fill), add_btn,].width(Length::Fill);

        let table_content = patients_table(&self.patients);

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

pub fn add_patient_form<'a>(draft: &'a DraftPatient) -> Element<'a, PatientsMessage> {
    let title = text("Patient Details").size(24);

    let name_input = text_input("Full Name", &draft.name)
        .on_input(|value| PatientsMessage::FieldChanged(PatientFormField::Name, value))
        .padding(10);

    // 1. The Date Picker Button
    let dob_display_text = if draft.birth_date.is_empty() {
        "Click to Select Date"
    } else {
        &draft.birth_date
    };

    let dob_btn = button(text(dob_display_text))
        .on_press(PatientsMessage::OpenDatePicker)
        .padding(10)
        .width(Length::Fill)
        .style(theme::secondary_button);

    let phone_input = text_input("Phone Number", &draft.phone)
        .on_input(|value| PatientsMessage::FieldChanged(PatientFormField::Phone, value))
        .padding(10);

    let address_input = text_input("Home Address", &draft.address)
        .on_input(|value| PatientsMessage::FieldChanged(PatientFormField::Address, value))
        .padding(10);

    let save_btn = button(text("Save Patient").align_x(Alignment::Center))
        .on_press(PatientsMessage::SubmitForm)
        .style(theme::primary_button)
        .padding([10, 20]);

    let cancel_btn = button(text("Cancel").align_x(Alignment::Center))
        .on_press(PatientsMessage::CloseForm)
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

    let initial_date = chrono::NaiveDate::parse_from_str(&draft.birth_date, "%Y-%m-%d")
        // If it's empty/invalid, open the calendar to Jan 1, 2000 (Great UX for birth dates!)
        .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    // 2. Wrap the form in the date_picker overlay
    date_picker(
        draft.show_picker,
        initial_date,
        form_container,
        PatientsMessage::CancelDatePicker,
        PatientsMessage::DateSelected,
    )
    .into()
}
