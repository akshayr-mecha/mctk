use bevy::{prelude::*, text::FontWeight};

use super::ThemeColors;

#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub color: Color,
    pub font_size: f32,
    pub font: Option<Handle<Font>>,
    pub font_weight: FontWeight,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub text_align: JustifyText,
}
impl TextStyle {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            color: colors.foreground,
            font_size: 16.0,
            font: None,
            font_weight: FontWeight::NORMAL,
            line_height: 1.5,
            letter_spacing: 0.0,
            text_align: JustifyText::Left,
        }
    }
}
