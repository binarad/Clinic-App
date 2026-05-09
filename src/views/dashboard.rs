use crate::database::models::Appointment;
use crate::database::operations::{
    fetch_all_employees_db, fetch_appointments_db, fetch_patient_db,
};
use crate::theme;

use iced::widget::{Space, column, container, row, table, text};
use iced::{Alignment, Element, Font, Length};

// ==========================================
// STATE & MESSAGES
// ==========================================
pub struct DashboardTab {
    pub total_patients: usize,
    pub total_staff: usize,
    // Store joined data for easy rendering: (Appointment, PatientName, DoctorName)
    pub upcoming_appointments: Vec<(Appointment, String, String)>,
}

impl Default for DashboardTab {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    Refresh,
}

// ==========================================
// COMPONENT LOGIC
// ==========================================
impl DashboardTab {
    pub fn new() -> Self {
        let mut tab = Self {
            total_patients: 0,
            total_staff: 0,
            upcoming_appointments: Vec::new(),
        };
        tab.sync_data();
        tab
    }

    /// Fetches fresh data and calculates dashboard stats
    fn sync_data(&mut self) {
        let patients = fetch_patient_db();
        let staff = fetch_all_employees_db();
        let mut appointments = fetch_appointments_db();

        self.total_patients = patients.len();
        self.total_staff = staff.len();

        let today = chrono::Local::now().date_naive();

        // 1. Filter for appointments that are today or in the future
        appointments.retain(|a| {
            if let Some(date_time) = a.appointment_date {
                date_time.date() >= today && a.status.as_deref() != Some("Cancelled")
            } else {
                false
            }
        });

        // 2. Sort them by date and time so the closest ones appear first
        appointments.sort_by(|a, b| {
            let date_cmp = a.appointment_date.cmp(&b.appointment_date);
            if date_cmp == std::cmp::Ordering::Equal {
                a.appointment_time.cmp(&b.appointment_time)
            } else {
                date_cmp
            }
        });

        // 3. Take the top 5 upcoming appointments and join the names
        self.upcoming_appointments = appointments
            .into_iter()
            .take(5)
            .map(|apt| {
                let patient_name = patients
                    .iter()
                    .find(|p| p.patient_id == apt.patient_id)
                    .map(|p| p.full_name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                let doctor_name = staff
                    .iter()
                    .find(|e| e.employee_id == apt.doctor_id)
                    .map(|e| e.full_name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                (apt, patient_name, doctor_name)
            })
            .collect();
    }

    pub fn update(&mut self, message: DashboardMessage) -> iced::Task<DashboardMessage> {
        match message {
            DashboardMessage::Refresh => {
                self.sync_data();
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        let header = text("Clinic Dashboard")
            .color(theme::NAVY_SLATE)
            .size(36.0)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            });

        // --- Summary Cards ---
        let patient_card = summary_card("Total Patients", self.total_patients);
        let staff_card = summary_card("Total Staff", self.total_staff);
        let apt_card = summary_card("Upcoming", self.upcoming_appointments.len());

        let stats_row = row![patient_card, staff_card, apt_card]
            .spacing(20)
            .width(Length::Fill);

        // --- Upcoming Appointments Section ---
        let subheader = text("Next 5 Upcoming Appointments").size(24.0).font(Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        });

        let content = column![
            header,
            Space::new().height(30.0),
            stats_row,
            Space::new().height(40.0),
            subheader,
            Space::new().height(15.0),
            upcoming_table(&self.upcoming_appointments)
        ]
        .spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40) // Extra padding for the dashboard layout
            .style(theme::main_background)
            .into()
    }
}

// ==========================================
// UI HELPERS
// ==========================================
fn summary_card<'a>(title: &'a str, count: usize) -> Element<'a, DashboardMessage> {
    let title_text = text(title)
        .size(18)
        .color(iced::Color::from_rgb(0.4, 0.4, 0.4));
    let count_text = text(count.to_string())
        .size(36)
        .font(Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .color(theme::NAVY_SLATE);

    let content =
        column![title_text, Space::new().height(5.0), count_text].align_x(Alignment::Center);

    container(content)
        .width(Length::FillPortion(1))
        .padding(20)
        .style(theme::white_card)
        .into()
}

fn upcoming_table<'a>(
    appointments: &'a [(Appointment, String, String)],
) -> Element<'a, DashboardMessage> {
    if appointments.is_empty() {
        return container(
            text("No upcoming appointments scheduled.")
                .size(18)
                .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
        )
        .width(Length::Fill)
        .padding(20)
        .center_x(Length::Fill)
        .into();
    }

    let columns = vec![
        table::column(
            text("Date & Time").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(a, _, _): &(Appointment, String, String)| {
                let date = a
                    .appointment_date
                    .map(|d| d.format("%b %d, %Y").to_string())
                    .unwrap_or_default();
                let time = a.appointment_time.clone().unwrap_or_default();
                Element::from(text(format!("{} at {}", date, time)))
            },
        )
        .width(Length::Fixed(200.0)),
        table::column(
            text("Patient").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(_, p_name, _): &(Appointment, String, String)| Element::from(text(p_name.clone())),
        )
        .width(Length::FillPortion(1)),
        table::column(
            text("Doctor").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(_, _, d_name): &(Appointment, String, String)| Element::from(text(d_name.clone())),
        )
        .width(Length::FillPortion(1)),
        table::column(
            text("Reason").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(a, _, _): &(Appointment, String, String)| {
                Element::from(text(a.reason.clone().unwrap_or_else(|| "N/A".to_string())))
            },
        )
        .width(Length::FillPortion(2)),
    ];

    let data_table = table(columns, appointments).padding(15.0).separator_y(1.0);

    container(data_table)
        .width(Length::Fill)
        .style(theme::white_card)
        .into()
}
