use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct PanelStyle {
    pub background_color: Color,
}

impl PanelStyle {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            background_color: colors.background,
        }
    }
}
