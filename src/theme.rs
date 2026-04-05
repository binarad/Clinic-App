use iced::widget::{button, container};
use iced::{Border, Color, Theme};

// ======================
//     PRIMARY COLORS
// ======================

// Main brand color: sidebar, navigation, primary action buttons
pub const DEEP_TEAL: Color = Color::from_rgb8(9, 76, 85);

// Secondary accent: active tab highlight, hover state for primary buttons
pub const MUTED_TEAL: Color = Color::from_rgb8(21, 109, 122);

// =============================
//     SURFACE & BACKGROUNDS
// =============================

// Base application background (outside the white cards)
pub const SOFT_CANVAS: Color = Color::from_rgb8(244, 247, 248);

// Data containers: tables, white card, dropdown menus, input fields
pub const CLEAN_WHITE: Color = Color::from_rgb8(255, 255, 255);

// ===============================
//     TYPOGRAPHY & STRUCTURAL
// ===============================

// Main typography: page titles, patient names, table text
pub const NAVY_SLATE: Color = Color::from_rgb8(30, 41, 59);

// Structural elements: divider lines, search borders, inactive icons
pub const SOFT_GRAY: Color = Color::from_rgb8(203, 213, 225);

// ==================================
//     SEMANTIC COLORS (STATUSES)
// ==================================

pub const STATUS_SCHEDULED_BG: Color = Color::from_rgb8(224, 242, 254);
pub const STATUS_SCHEDULED_TEXT: Color = Color::from_rgb8(3, 105, 161);

pub const STATUS_COMPLETED_BG: Color = Color::from_rgb8(220, 252, 231);
pub const STATUS_COMPLETED_TEXT: Color = Color::from_rgb8(21, 128, 61);

pub const STATUS_NO_SHOW_BG: Color = Color::from_rgb8(254, 226, 226);
pub const STATUS_NO_SHOW_TEXT: Color = Color::from_rgb8(185, 28, 28);

// =====================
//     WIDGET STYLES
// =====================

// Style for the main application canvas
pub fn main_background(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(NAVY_SLATE),
        background: Some(iced::Background::Color(SOFT_CANVAS)),
        border: Border::default(),
        ..Default::default()
    }
}

// Style for data tables and elevated UI cards
pub fn white_card(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(NAVY_SLATE),
        background: Some(iced::Background::Color(CLEAN_WHITE)),
        border: Border {
            color: SOFT_GRAY,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

// Style for primary action buttons (e.g., "+ New Appointment")
pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(MUTED_TEAL)),
            text_color: CLEAN_WHITE,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },

        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(SOFT_GRAY)),
            text_color: CLEAN_WHITE,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },

        _ => button::Style {
            background: Some(iced::Background::Color(DEEP_TEAL)),
            text_color: CLEAN_WHITE,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

// Style for semantic status badges (e.g., "Completed")
pub fn badge_completed(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(STATUS_COMPLETED_TEXT),
        background: Some(iced::Background::Color(STATUS_COMPLETED_BG)),
        border: Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn badge_no_show(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(STATUS_NO_SHOW_TEXT),
        background: Some(iced::Background::Color(STATUS_NO_SHOW_BG)),
        border: Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn badge_sheduled(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(STATUS_SCHEDULED_TEXT),
        background: Some(iced::Background::Color(STATUS_SCHEDULED_BG)),
        border: Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
