use crate::theme;
use iced::widget::{Space, button, column, container, pick_list, row, table, text, text_input};
use iced::{Alignment, Element, Font, Length};
// use iced_aw::{badge, drop_down};

use crate::database::models::Employee;

#[derive(Debug, Clone)]
pub enum EmployeesMessage {
    OpenAddForm,
    CloseAddForm,

    // Form inputs
    NameChanged(String),
    PhoneChanged(String),
    EmailChanged(String),
    RoleSelected(String),

    // Async Actions
    SubmitForm,
    EmployeeAdded(Result<Employee, String>),
    // SearchBarContentChanged(String),
}

const ROLES: &[&str] = &["Doctor", "Nurse", "Admin", "Registrar"];

pub fn view<'a>(employees: &'a [Employee]) -> Element<'a, EmployeesMessage> {
    // let mut employees_page = column!().spacing(10.0).padding(20.0);
    let header = row![text("Employees").color(theme::NAVY_SLATE).size(30.0)]
        .align_y(Alignment::Center)
        .width(Length::Fill);
    // let tools_row; // Add search_bar and the others dropdown and button
    let add_btn = button(text("Add New Employee"))
        .on_press(EmployeesMessage::OpenAddForm)
        .style(theme::primary_button)
        .padding([4, 8]);

    let action_bar = row![
        Space::new().width(Length::Fill),
        add_btn,
        // Space::with_width(Length::Fill), // Un-comment later to push search bar to the right
        // search_input
    ]
    .width(Length::Fill);
    let table_content = employees_table(employees);
    // let footer = row![text(employees.len())];

    let content = column![
        header,
        Space::new().height(5.0),
        action_bar,
        Space::new().height(15.0),
        table_content,
    ]
    .spacing(0);
    // employees_page = employees_page.push(header);
    // employees_page = employees_page.push(table_content);
    // employees_page = employees_page.push(footer);

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
            |_employee: &Employee| -> Element<'_, EmployeesMessage> {
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

    let total_count = employees.len();
    let footer = container(
        text(format!("Total Employees: {}", total_count)).font(Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }),
    )
    .width(Length::Fill)
    .align_x(Alignment::Start)
    .padding([10, 20]);

    let empty_space = Space::new(); // IDC
    let content = column![data_table, empty_space.height(Length::Fill), footer,];
    // Wrap it in a container for styling
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::white_card)
        .into()
}

// =================
// ADD EMPLOYEE FORM
// =================

// TODO ADD VALIDATION TO THE FORM FIELDS
pub fn add_employee_form<'a>(
    draft_name: &str,
    draft_phone: &str,
    draft_email: &str,
    selected_role: Option<&'a str>,
) -> Element<'a, EmployeesMessage> {
    let title = text("Add New Employee").size(24);

    let name_input = text_input("Full Name", draft_name)
        .on_input(EmployeesMessage::NameChanged)
        .padding(10);

    let phone_input = text_input("Phone Number", draft_phone)
        .on_input(EmployeesMessage::PhoneChanged)
        .padding(10);

    let email_input = text_input("Email Address", draft_email)
        .on_input(EmployeesMessage::EmailChanged)
        .padding(10);

    let role_dropdown = pick_list(ROLES, selected_role, |role| {
        EmployeesMessage::RoleSelected(role.to_string())
    })
    .placeholder("Select a Role")
    .padding(10)
    .width(Length::Fill);

    let save_btn = button(text("Save Employee").align_x(Alignment::Center))
        .on_press(EmployeesMessage::SubmitForm)
        .style(theme::primary_button)
        .padding([10, 20]);

    let cancel_btn = button(text("Cancel").align_x(Alignment::Center))
        .on_press(EmployeesMessage::CloseAddForm)
        .style(theme::secondary_button)
        .padding([10, 20]);
    // Todo add Cancel button

    let actions = row![save_btn, cancel_btn].spacing(15); // Cancel button goes here

    let form_content = column![
        title,
        Space::new().width(20.0),
        text("Full Name").size(14),
        name_input,
        Space::new().height(10.0),
        text("Role").size(14),
        role_dropdown,
        Space::new().height(10.0),
        text("Phone").size(14),
        phone_input,
        Space::new().height(10.0),
        text("Email").size(14),
        email_input,
        Space::new().height(10.0),
        actions
    ]
    .spacing(5)
    .max_width(500.0);

    container(form_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30)
        .style(theme::main_background)
        .into()
}
