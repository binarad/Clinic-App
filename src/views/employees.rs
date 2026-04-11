use crate::theme;
use iced::widget::{button, column, container, row, table, text};
use iced::{Alignment, Element, Font, Length};
// use iced_aw::{badge, drop_down};

use crate::database::models::Employee;

#[derive(Debug, Clone)]
pub enum EmployeesMessage {
    // SearchBarContentChanged(String),
}

pub fn view<'a>(employees: &'a [Employee]) -> Element<'a, EmployeesMessage> {
    let mut employees_page = column!().spacing(10.0).padding(20.0);
    let header = row![text("Employees").color(theme::NAVY_SLATE).size(30.0)]
        .align_y(Alignment::Center)
        .width(Length::Fill);
    // let tools_row; // Add search_bar and the others dropdown and button
    let table_content = employees_table(employees);
    let footer = row![text(employees.len())];

    employees_page = employees_page.push(header);
    employees_page = employees_page.push(table_content);
    employees_page = employees_page.push(footer);

    container(employees_page)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::main_background)
        .into()
}

pub fn employees_table<'a>(employees: &'a [Employee]) -> Element<'a, EmployeesMessage> {
    // 1. Define the Columns
    let columns = vec![
        // Column 1: ID
        table::column(
            text("ID").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, EmployeesMessage> {
                Element::from(text(employee.employee_id.to_string()))
            },
        )
        .width(Length::Fixed(50.0)),
        // Column 2: Full Name
        table::column(
            text("Full Name").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, EmployeesMessage> {
                Element::from(text(&employee.full_name))
            },
        )
        .width(Length::FillPortion(2)),
        // Column 3 Role
        table::column(
            text("Role").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, EmployeesMessage> {
                let badge = container(text(&employee.role))
                    .align_x(Alignment::Center)
                    .padding([4, 12]);

                let styled_badge = match employee.role.as_str() {
                    "Doctor" => badge.style(theme::badge_doctor),
                    "Nurse" => badge.style(theme::badge_nurse),
                    "Admin" => badge.style(theme::badge_admin),
                    "Registrar" => badge.style(theme::badge_registrar),
                    _ => badge.style(theme::badge_admin),
                };

                Element::from(styled_badge)
            },
        )
        .width(Length::Fixed(100.0)),
        // Column 4: Phone
        table::column(
            text("Phone").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, EmployeesMessage> {
                let phone_text = employee.phone.as_deref().unwrap_or("N/A");
                Element::from(text(phone_text))
            },
        )
        .width(Length::Fixed(120.0)),
        // Column 5: Email
        table::column(
            text("Email").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, EmployeesMessage> {
                let email_text = employee.email.as_deref().unwrap_or("N/A");
                Element::from(
                    text(email_text)
                        .width(Length::Fill)
                        .wrapping(text::Wrapping::WordOrGlyph),
                )
            },
        )
        .width(Length::FillPortion(3)),
        // Column 6: Actions (The buttons on the right)
        table::column(
            text("Actions").font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            |employee: &Employee| -> Element<'_, EmployeesMessage> {
                let edit_btn = button("Edit"); // TODO: add on_press action for buttons
                let delete_btn = button("Delete");

                row![edit_btn, delete_btn].spacing(5).into()
            },
        )
        .width(Length::Fixed(150.0)),
    ];

    // 2. Build the actual table using the columns and the raw data rows
    let data_table = table(columns, employees)
        .padding(10.0)
        .separator_y(1.0)
        .separator_x(0.0);

    // Wrap it in a container for styling
    container(data_table)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::white_card)
        .into()
}
