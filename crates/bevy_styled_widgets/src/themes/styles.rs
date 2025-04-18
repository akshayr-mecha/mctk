use bevy::prelude::*;

use super::{
    ThemeColors,
    button::{ButtonSizeStyles, ButtonVariantStyles, button_sizes},
    panel::PanelStyle,
    text::TextStyle,
};

#[derive(Debug, Clone)]
pub struct ThemeStyles {
    pub buttons: ButtonVariantStyles,
    pub button_sizes: ButtonSizeStyles,
    pub text: TextStyle,
    pub panel: PanelStyle,
}

impl ThemeStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            buttons: ButtonVariantStyles::from_colors(colors.clone()),
            button_sizes: button_sizes(),
            text: TextStyle::from_colors(colors.clone()),
            panel: PanelStyle::from_colors(colors.clone()),
        }
    }
}
