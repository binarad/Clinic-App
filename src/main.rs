use iced::Element;
use iced::widget::{container, row};

use crate::components::sidebar::{self, Tab};
use crate::views::appointments::{AppointmentsMessage, AppointmentsTab};
use crate::views::dashboard::{DashboardMessage, DashboardTab};
use crate::views::employees::{EmployeesMessage, EmployeesTab};
use crate::views::patients::{PatientsMessage, PatientsTab};
use crate::views::registry::{RegistryMessage, RegistryTab};

pub mod components;
pub mod database;
pub mod theme;
pub mod views;

#[derive(Debug, Clone)]
enum Message {
    Sidebar(sidebar::Message),
    DashboardView(DashboardMessage),
    EmployeeView(EmployeesMessage),
    PatientView(PatientsMessage),
    AppointmentView(AppointmentsMessage),
    RegistryView(RegistryMessage),
}

pub struct ClinicApp {
    pub dashboard_tab: DashboardTab,
    pub patients_tab: PatientsTab,
    pub employees_tab: EmployeesTab,
    pub appointments_tab: AppointmentsTab,
    pub registry_tab: RegistryTab,

    pub active_tab: Tab,
}

impl ClinicApp {
    fn new() -> Self {
        Self {
            dashboard_tab: DashboardTab::new(),
            patients_tab: PatientsTab::new(),
            employees_tab: EmployeesTab::new(),
            appointments_tab: AppointmentsTab::new(),
            registry_tab: RegistryTab::new(),

            active_tab: Tab::Dashboard,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Sidebar(sidebar::Message::SelectedTab(new_tab)) => {
                self.active_tab = new_tab;
            }

            // DASHBOARD VIEW
            Message::DashboardView(msg) => {
                return self.dashboard_tab.update(msg).map(Message::DashboardView);
            }

            // EMPLOYEES VIEW
            Message::EmployeeView(msg) => {
                let needs_registry_refresh = matches!(
                    &msg,
                    EmployeesMessage::EmployeeAdded(Ok(_))
                        | EmployeesMessage::DeletedEmployee(Ok(_), _)
                );

                // // 2. Let the Employee tab process the message normally
                let task = self.employees_tab.update(msg).map(Message::EmployeeView);

                // 3. If a change happened, force the Registry tab to refresh its data
                if needs_registry_refresh {
                    // This synchronously re-fetches the joined data from the DB!
                    let _ = self
                        .registry_tab
                        .update(crate::views::registry::RegistryMessage::Refresh);
                }

                return task;
            }

            // PATIENTS VIEW
            Message::PatientView(msg) => {
                return self.patients_tab.update(msg).map(Message::PatientView);
            }

            // APPOINTMENTS VIEW
            Message::AppointmentView(msg) => {
                let needs_dash_refresh = matches!(
                    &msg,
                    AppointmentsMessage::AppointmentAdded(Ok(_))
                        | AppointmentsMessage::DeletedAppointment(Ok(_), _)
                );

                let task = self
                    .appointments_tab
                    .update(msg)
                    .map(Message::AppointmentView);
                if needs_dash_refresh {
                    let _ = self.dashboard_tab.update(DashboardMessage::Refresh);
                }

                return task;
            }

            // REGISTRY VIEW
            Message::RegistryView(msg) => {
                return self.registry_tab.update(msg).map(Message::RegistryView);
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let sidebar_view = sidebar::view(&self.active_tab).map(Message::Sidebar);

        // 2. Render the main content area dynamically
        let content_view: Element<Message> = match self.active_tab {
            Tab::Dashboard => self.dashboard_tab.view().map(Message::DashboardView),
            Tab::Patients => self.patients_tab.view().map(Message::PatientView),
            Tab::Employees => self.employees_tab.view().map(Message::EmployeeView),
            Tab::Appointments => self
                .appointments_tab
                .view(&self.patients_tab.patients, &self.employees_tab.employees)
                .map(Message::AppointmentView),
            Tab::Registry => self.registry_tab.view().map(Message::RegistryView),
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
