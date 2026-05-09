use crate::components::shared::pagination::pagination;
use crate::components::shared::search_bar::search_bar;
use crate::database::models::{Employee, RegistryWorker};
use crate::database::operations::fetch_registry_joined_db;
use crate::theme;

use iced::widget::{Space, column, container, row, table, text};
use iced::{Alignment, Element, Font, Length};

const ITEMS_PER_PAGE: usize = 10;

// ==========================================
// STATE & MESSAGES
// ==========================================
pub struct RegistryTab {
    pub workers: Vec<(Employee, RegistryWorker)>,
    pub search_query: String,
    pub current_page: usize,
    pub page_data: Vec<(Employee, RegistryWorker)>,
    pub total_pages: usize,
}

impl Default for RegistryTab {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum RegistryMessage {
    Refresh,
    SearchChanged(String),
    ClearSearch,
    NextPage,
    PreviousPage,
}

// ==========================================
// COMPONENT LOGIC
// ==========================================
impl RegistryTab {
    pub fn new() -> Self {
        let mut tab = Self {
            workers: fetch_registry_joined_db(),
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

        let filtered: Vec<&(Employee, RegistryWorker)> = self
            .workers
            .iter()
            .filter(|(e, r)| {
                if query.is_empty() {
                    return true;
                }
                let name_matches = e.full_name.to_lowercase().contains(&query);
                let phone_matches = e.phone.as_deref().unwrap_or("").contains(&query);
                let window_matches = r
                    .window_number
                    .map(|w| w.to_string())
                    .unwrap_or_default()
                    .contains(&query);
                name_matches || phone_matches || window_matches
            })
            .collect();

        let total_items = filtered.len();
        self.total_pages = (total_items as f32 / ITEMS_PER_PAGE as f32).ceil() as usize;
        self.current_page = self.current_page.min(self.total_pages.max(1));

        let start_idx = (self.current_page.saturating_sub(1)) * ITEMS_PER_PAGE;
        let end_idx = (start_idx + ITEMS_PER_PAGE).min(total_items);

        self.page_data = filtered[start_idx..end_idx]
            .iter()
            .map(|&t| t.clone())
            .collect();
    }

    pub fn update(&mut self, message: RegistryMessage) -> iced::Task<RegistryMessage> {
        match message {
            RegistryMessage::SearchChanged(query) => {
                self.search_query = query;
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            RegistryMessage::ClearSearch => {
                self.search_query.clear();
                self.current_page = 1;
                self.sync_view();
                iced::Task::none()
            }
            RegistryMessage::NextPage => {
                self.current_page += 1;
                self.sync_view();
                iced::Task::none()
            }
            RegistryMessage::PreviousPage => {
                if self.current_page > 1 {
                    self.current_page -= 1;
                    self.sync_view();
                }
                iced::Task::none()
            }
            RegistryMessage::Refresh => {
                self.workers = fetch_registry_joined_db();
                self.sync_view(); // Re-sync after fetching new data!
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

        let search_ui = search_bar(
            &self.search_query,
            "Search name, phone, or window...",
            RegistryMessage::SearchChanged,
            Some(RegistryMessage::ClearSearch),
        );

        let pagination_ui = pagination(
            self.current_page,
            self.total_pages,
            RegistryMessage::PreviousPage,
            RegistryMessage::NextPage,
        );

        let footer_text = text(format!("Total Registry Workers: {}", self.workers.len()))
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
            search_ui,
            Space::new().height(15.0),
            registry_table(&self.page_data),
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
