use iced::Element;
use iced::widget::{container, row, text};

use crate::components::sidebar::{self, Tab};
use crate::views::appointments::{AppointmentsMessage, AppointmentsTab};
use crate::views::employees::{EmployeesMessage, EmployeesTab};
use crate::views::patients::{PatientsMessage, PatientsTab};

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

// Helper to grab the mutable draft during updates

#[derive(Debug, Clone)]
enum Message {
    Sidebar(sidebar::Message),
    EmployeeView(EmployeesMessage),
    PatientView(PatientsMessage),
    AppointmentView(AppointmentsMessage),
}

pub struct ClinicApp {
    pub patients_tab: PatientsTab,
    pub employees_tab: EmployeesTab,
    pub appointments_tab: AppointmentsTab,

    pub active_tab: Tab,
}

impl ClinicApp {
    fn new() -> Self {
        Self {
            patients_tab: PatientsTab::new(),
            employees_tab: EmployeesTab::new(),
            appointments_tab: AppointmentsTab::new(),

            active_tab: Tab::Dashboard,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Sidebar(sidebar::Message::SelectedTab(new_tab)) => {
                self.active_tab = new_tab;
            }

            // =====================================
            // EMPLOYEES VIEW
            // =====================================
            Message::EmployeeView(msg) => {
                return self.employees_tab.update(msg).map(Message::EmployeeView);
            }

            // =====================================
            // PATIENTS VIEW
            // =====================================
            Message::PatientView(msg) => {
                return self.patients_tab.update(msg).map(Message::PatientView);
            }

            // =====================================
            // APPOINTMENTS VIEW
            // =====================================
            Message::AppointmentView(msg) => {
                return self
                    .appointments_tab
                    .update(msg)
                    .map(Message::AppointmentView);
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let sidebar_view = sidebar::view(&self.active_tab).map(Message::Sidebar);

        // 2. Render the main content area dynamically
        let content_view: Element<Message> = match self.active_tab {
            Tab::Dashboard => text("Dashboard View").size(30).into(),

            // Map the patient table!
            Tab::Patients => self.patients_tab.view().map(Message::PatientView),

            Tab::Employees => self.employees_tab.view().map(Message::EmployeeView),
            Tab::Appointments => self
                .appointments_tab
                .view(&self.patients_tab.patients, &self.employees_tab.employees)
                .map(Message::AppointmentView),
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
