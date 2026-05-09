use crate::components::shared::search_bar::search_bar;
use crate::database::models::{MedicalRecord, NewRecordEntry, Patient, RecordEntry};
use crate::database::operations::{
    delete_record_entry_db, fetch_all_medical_records_db, fetch_entries_for_record_db,
    fetch_patient_db, insert_medical_record_db, insert_record_entry_db, update_record_entry_db,
};
use crate::theme;

use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Font, Length};

// ==========================================
// STATE & DRAFTS
// ==========================================
pub struct MedicalRecordsTab {
    pub patients: Vec<Patient>,
    pub medical_records: Vec<MedicalRecord>,

    // Master-Detail State
    pub search_query: String,
    pub selected_patient: Option<Patient>,
    pub selected_record: Option<MedicalRecord>,
    pub current_entries: Vec<RecordEntry>,

    // Form State
    pub show_add_entry_form: bool,
    pub editing_entry_id: Option<i32>, // Tracks if we are editing an existing entry!
    pub draft_diagnosis: String,
    pub draft_complaints: String,
    pub is_saving: bool,
}

impl Default for MedicalRecordsTab {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum RecordsMessage {
    SearchChanged(String),
    ClearSearch,
    SelectPatient(Patient),

    // Record Creation
    CreateRecordForPatient(i32),
    RecordCreated(Result<MedicalRecord, String>),

    // Entry Creation & Editing
    OpenAddEntryForm,
    OpenEditEntryForm(RecordEntry), // Populates draft with existing data
    CancelAddEntry,
    DiagnosisChanged(String),
    ComplaintsChanged(String),
    SubmitEntry,
    EntryAdded(Result<RecordEntry, String>),
    EntryUpdated(Result<RecordEntry, String>),

    // Entry Deletion
    DeleteEntry(i32),
    EntryDeleted(Result<usize, String>, i32),
}

// ==========================================
// COMPONENT LOGIC
// ==========================================
impl MedicalRecordsTab {
    #[must_use] 
    pub fn new() -> Self {
        Self {
            patients: fetch_patient_db(),
            medical_records: fetch_all_medical_records_db(),
            search_query: String::new(),
            selected_patient: None,
            selected_record: None,
            current_entries: Vec::new(),
            show_add_entry_form: false,
            editing_entry_id: None,
            draft_diagnosis: String::new(),
            draft_complaints: String::new(),
            is_saving: false,
        }
    }

    pub fn update(&mut self, message: RecordsMessage) -> iced::Task<RecordsMessage> {
        match message {
            RecordsMessage::SearchChanged(q) => {
                self.search_query = q;
                iced::Task::none()
            }
            RecordsMessage::ClearSearch => {
                self.search_query.clear();
                iced::Task::none()
            }
            RecordsMessage::SelectPatient(patient) => {
                self.selected_patient = Some(patient.clone());
                self.show_add_entry_form = false;
                self.editing_entry_id = None;

                self.selected_record = self
                    .medical_records
                    .iter()
                    .find(|r| r.patient_id == patient.patient_id)
                    .cloned();

                if let Some(record) = &self.selected_record {
                    self.current_entries = fetch_entries_for_record_db(record.record_number);
                } else {
                    self.current_entries.clear();
                }

                iced::Task::none()
            }
            RecordsMessage::CreateRecordForPatient(patient_id) => {
                self.is_saving = true;
                iced::Task::perform(
                    insert_medical_record_db(patient_id),
                    RecordsMessage::RecordCreated,
                )
            }
            RecordsMessage::RecordCreated(result) => {
                self.is_saving = false;
                if let Ok(new_record) = result {
                    self.medical_records.push(new_record.clone());
                    self.selected_record = Some(new_record);
                    self.current_entries.clear();
                }
                iced::Task::none()
            }
            RecordsMessage::OpenAddEntryForm => {
                self.show_add_entry_form = true;
                self.editing_entry_id = None;
                self.draft_diagnosis.clear();
                self.draft_complaints.clear();
                iced::Task::none()
            }
            RecordsMessage::OpenEditEntryForm(entry) => {
                self.show_add_entry_form = true;
                self.editing_entry_id = Some(entry.entry_id);
                self.draft_diagnosis = entry.diagnosis.unwrap_or_default();
                self.draft_complaints = entry.complaints.unwrap_or_default();
                iced::Task::none()
            }
            RecordsMessage::CancelAddEntry => {
                self.show_add_entry_form = false;
                self.editing_entry_id = None;
                iced::Task::none()
            }
            RecordsMessage::DiagnosisChanged(v) => {
                self.draft_diagnosis = v;
                iced::Task::none()
            }
            RecordsMessage::ComplaintsChanged(v) => {
                self.draft_complaints = v;
                iced::Task::none()
            }
            RecordsMessage::SubmitEntry => {
                if let Some(record) = &self.selected_record {
                    if self.draft_diagnosis.trim().is_empty() {
                        return iced::Task::none();
                    }

                    self.is_saving = true;

                    if let Some(entry_id) = self.editing_entry_id {
                        // We are EDITING an existing entry
                        iced::Task::perform(
                            update_record_entry_db(
                                entry_id,
                                self.draft_diagnosis.clone(),
                                self.draft_complaints.clone(),
                            ),
                            RecordsMessage::EntryUpdated,
                        )
                    } else {
                        // We are ADDING a new entry
                        let payload = NewRecordEntry {
                            record_number: record.record_number,
                            entry_number: None,
                            entry_date: Some(chrono::Local::now().naive_local()),
                            diagnosis: Some(self.draft_diagnosis.clone()),
                            complaints: Some(self.draft_complaints.clone()),
                        };

                        iced::Task::perform(
                            insert_record_entry_db(payload),
                            RecordsMessage::EntryAdded,
                        )
                    }
                } else {
                    iced::Task::none()
                }
            }
            RecordsMessage::EntryAdded(result) => {
                self.is_saving = false;
                if let Ok(new_entry) = result {
                    self.current_entries.insert(0, new_entry);
                    self.show_add_entry_form = false;
                }
                iced::Task::none()
            }
            RecordsMessage::EntryUpdated(result) => {
                self.is_saving = false;
                if let Ok(updated_entry) = result {
                    if let Some(idx) = self
                        .current_entries
                        .iter()
                        .position(|e| e.entry_id == updated_entry.entry_id)
                    {
                        self.current_entries[idx] = updated_entry;
                    }
                    self.show_add_entry_form = false;
                    self.editing_entry_id = None;
                }
                iced::Task::none()
            }
            RecordsMessage::DeleteEntry(id) => {
                iced::Task::perform(delete_record_entry_db(id), move |res| {
                    RecordsMessage::EntryDeleted(res, id)
                })
            }
            RecordsMessage::EntryDeleted(result, deleted_id) => {
                if result.is_ok() {
                    self.current_entries.retain(|e| e.entry_id != deleted_id);
                }
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, RecordsMessage> {
        // --- 1. LEFT SIDEBAR (PATIENT LIST) ---
        let query = self.search_query.to_lowercase();
        let filtered_patients: Vec<&Patient> = self
            .patients
            .iter()
            .filter(|p| query.is_empty() || p.full_name.to_lowercase().contains(&query))
            .collect();

        let search_ui = search_bar(
            &self.search_query,
            "Search patients...",
            RecordsMessage::SearchChanged,
            Some(RecordsMessage::ClearSearch),
        );

        let mut patient_list = column![].spacing(5);
        for p in filtered_patients {
            let is_selected =
                self.selected_patient.as_ref().map(|sp| sp.patient_id) == Some(p.patient_id);

            let btn_style = if is_selected {
                theme::primary_button
            } else {
                theme::secondary_button
            };

            patient_list = patient_list.push(
                button(text(p.full_name.clone()).size(16).width(Length::Fill))
                    .style(btn_style)
                    .padding(10)
                    .width(Length::Fill)
                    .on_press(RecordsMessage::SelectPatient(p.clone())),
            );
        }

        let sidebar = container(
            column![
                text("Patient Records")
                    .size(24)
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(theme::NAVY_SLATE),
                Space::new().height(15.0),
                search_ui,
                Space::new().height(10.0),
                scrollable(patient_list).height(Length::Fill)
            ]
            .spacing(0),
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(20)
        .style(theme::white_card);

        // --- 2. RIGHT SIDE (DETAILS VIEW) ---
        let detail_view: Element<'_, RecordsMessage> = match &self.selected_patient {
            None => container(
                text("Select a patient from the list to view their medical record.")
                    .size(18)
                    .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
            Some(patient) => match &self.selected_record {
                None => {
                    let content = column![
                        text(format!("No Medical Record found for {}", patient.full_name)).size(20),
                        Space::new().height(20.0),
                        button(text("Initialize Medical Record"))
                            .style(theme::primary_button)
                            .padding([10, 20])
                            .on_press(RecordsMessage::CreateRecordForPatient(patient.patient_id))
                    ]
                    .align_x(Alignment::Center);

                    container(content)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                        .into()
                }
                Some(record) => {
                    let header = row![
                        column![
                            text(patient.full_name.clone())
                                .size(28)
                                .font(Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Default::default()
                                })
                                .color(theme::NAVY_SLATE),
                            text(format!("Record #{:05}", record.record_number))
                                .size(16)
                                .color(iced::Color::from_rgb(0.5, 0.5, 0.5))
                        ],
                        Space::new().width(Length::Fill),
                        if self.show_add_entry_form {
                            button(text("Cancel"))
                                .style(theme::secondary_button)
                                .padding([8, 15])
                                .on_press(RecordsMessage::CancelAddEntry)
                        } else {
                            button(text("+ Add Entry"))
                                .style(theme::primary_button)
                                .padding([8, 15])
                                .on_press(RecordsMessage::OpenAddEntryForm)
                        }
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                    let inner_content: Element<'_, RecordsMessage> = if self.show_add_entry_form {
                        let form_title = if self.editing_entry_id.is_some() {
                            "Edit Medical Entry"
                        } else {
                            "New Medical Entry"
                        };

                        column![
                            text(form_title).size(20).font(Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            }),
                            Space::new().height(15.0),
                            text("Patient Complaints").size(14),
                            text_input("Describe symptoms...", &self.draft_complaints)
                                .on_input(RecordsMessage::ComplaintsChanged)
                                .padding(10),
                            Space::new().height(15.0),
                            text("Diagnosis / Notes").size(14),
                            text_input("Enter diagnosis...", &self.draft_diagnosis)
                                .on_input(RecordsMessage::DiagnosisChanged)
                                .padding(10),
                            Space::new().height(20.0),
                            button(text("Save Entry to File"))
                                .style(theme::primary_button)
                                .padding([10, 20])
                                .on_press(RecordsMessage::SubmitEntry)
                        ]
                        .into()
                    } else {
                        let mut entries_list = column![].spacing(15);
                        if self.current_entries.is_empty() {
                            entries_list =
                                entries_list.push(text("No entries found in this record yet."));
                        } else {
                            for entry in &self.current_entries {
                                let date_str = entry
                                    .entry_date.map_or_else(|| "Unknown Date".to_string(), |d| d.format("%B %d, %Y - %H:%M").to_string());

                                let card = container(column![
                                    row![
                                        text(date_str)
                                            .size(14)
                                            .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                                        Space::new().width(Length::Fill),
                                        button("Edit")
                                            .style(theme::primary_button)
                                            .padding([4, 12])
                                            .on_press(RecordsMessage::OpenEditEntryForm(
                                                entry.clone()
                                            )),
                                        button("Delete")
                                            .style(theme::secondary_button)
                                            .padding([4, 12])
                                            .on_press(RecordsMessage::DeleteEntry(entry.entry_id))
                                    ]
                                    .align_y(Alignment::Center),
                                    Space::new().height(10.0),
                                    text(format!(
                                        "Diagnosis: {}",
                                        entry.diagnosis.clone().unwrap_or_default()
                                    ))
                                    .font(Font {
                                        weight: iced::font::Weight::Bold,
                                        ..Default::default()
                                    })
                                    .size(18),
                                    Space::new().height(5.0),
                                    text(format!(
                                        "Complaints: {}",
                                        entry.complaints.clone().unwrap_or_default()
                                    ))
                                    .size(16)
                                ])
                                .width(Length::Fill)
                                .padding(15)
                                .style(theme::white_card);

                                entries_list = entries_list.push(card);
                            }
                        }
                        scrollable(entries_list).height(Length::Fill).into()
                    };

                    column![header, Space::new().height(20.0), inner_content].into()
                }
            },
        };

        let detail_container = container(detail_view)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(30);

        // --- 3. ASSEMBLE SPLIT SCREEN ---
        let split_screen = row![sidebar, detail_container]
            .width(Length::Fill)
            .height(Length::Fill);

        container(split_screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(theme::main_background)
            .into()
    }
}
