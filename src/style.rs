use iced::{Background, Border, Color, color, widget::container::Style};

pub fn with_border(background: impl Into<Background>) -> Style {
    Style {
        border: Border::default().width(2).color(Color::WHITE),
        ..Style::default().background(background)
    }
}

pub fn blue() -> Color {
    Color::from_rgba(0.3, 0.5, 0.9, 0.8)
}

pub fn light_blue() -> Color {
    color!(229, 244, 255)
}

pub fn light_gray() -> Color {
    color!(241, 243, 245)
}

pub fn green() -> Color {
    Color::from_rgba(0.3, 0.5, 0.9, 0.8)
}
pub fn orange() -> Color {
    Color::from_rgba(0.3, 0.5, 0.9, 0.8)
}

pub fn light_orange() -> Color {
    color!(96, 40, 30)
}
pub fn purple() -> Color {
    Color::from_rgba(0.3, 0.5, 0.9, 0.8)
}
pub fn grau() -> Color {
    Color::from_rgba(0.3, 0.5, 0.9, 0.8)
}
