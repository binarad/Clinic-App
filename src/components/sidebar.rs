use iced::widget::{button, column, container, text};
use iced::{Element, Length};

use crate::theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Patients,
    Employees,
    Appointments,
    Registry,
}
#[derive(Debug, Clone)]
pub enum Message {
    SelectedTab(Tab),
}

pub fn view(active_tab: &Tab) -> Element<'_, Message> {
    let tabs = [
        (Tab::Dashboard, "Home Dashboard"),
        (Tab::Patients, "Patients"),
        (Tab::Employees, "Employees"),
        (Tab::Appointments, "Appointments"),
        (Tab::Registry, "Registry"),
    ];

    let mut sidebar_column = column!().spacing(10.0).padding(20.0);

    for (tab, label) in tabs {
        let is_active = *active_tab == tab;

        let btn = button(text(label))
            .width(Length::Fill)
            .padding(12)
            .on_press(Message::SelectedTab(tab))
            .style(move |_theme, status| {
                if is_active {
                    button::Style {
                        background: Some(iced::Background::Color(theme::MUTED_TEAL)),
                        text_color: theme::CLEAN_WHITE,
                        border: iced::Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                } else {
                    theme::primary_button(_theme, status)
                }
            });

        sidebar_column = sidebar_column.push(btn);
    }
    container(sidebar_column)
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(theme::DEEP_TEAL)),
            ..Default::default()
        })
        .into()
}
