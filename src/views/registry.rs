use crate::database::models::{Employee, RegistryWorker};
use crate::database::operations::fetch_registry_joined_db;
use crate::theme;
use iced::widget::{Space, column, container, row, table, text};
use iced::{Alignment, Element, Font, Length};

// ==========================================
// STATE & MESSAGES
// ==========================================
pub struct RegistryTab {
    pub workers: Vec<(Employee, RegistryWorker)>,
}

impl Default for RegistryTab {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum RegistryMessage {
    Refresh,
}

// ==========================================
// COMPONENT LOGIC
// ==========================================
impl RegistryTab {
    pub fn new() -> Self {
        Self {
            workers: fetch_registry_joined_db(),
        }
    }

    pub fn update(&mut self, message: RegistryMessage) -> iced::Task<RegistryMessage> {
        match message {
            RegistryMessage::Refresh => {
                // If you add a new worker in the Employees tab,
                // this lets the Registry tab refresh its list!
                self.workers = fetch_registry_joined_db();
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, RegistryMessage> {
        let header = row![
            text("Registry & Front Desk")
                .color(theme::NAVY_SLATE)
                .size(30.0)
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        let content = column![
            header,
            Space::new().height(20.0),
            registry_table(&self.workers)
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
fn registry_table<'a>(workers: &'a [(Employee, RegistryWorker)]) -> Element<'a, RegistryMessage> {
    let columns = vec![
        table::column(
            text("ID").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(e, _): &(Employee, RegistryWorker)| Element::from(text(e.employee_id.to_string())),
        )
        .width(Length::Fixed(50.0)),
        table::column(
            text("Name").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(e, _): &(Employee, RegistryWorker)| Element::from(text(e.full_name.clone())),
        )
        .width(Length::FillPortion(2)),
        table::column(
            text("Window Number").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(_, r): &(Employee, RegistryWorker)| {
                Element::from(text(
                    r.window_number
                        .map(|w| w.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                ))
            },
        )
        .width(Length::Fixed(150.0)),
        table::column(
            text("Phone").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |(e, _): &(Employee, RegistryWorker)| {
                Element::from(text(e.phone.clone().unwrap_or_else(|| "N/A".to_string())))
            },
        )
        .width(Length::FillPortion(2)),
    ];

    let data_table = table(columns, workers).padding(10.0).separator_y(1.0);
    container(data_table)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::white_card)
        .into()
}
