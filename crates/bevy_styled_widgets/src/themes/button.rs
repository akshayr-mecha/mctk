use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    // Colors
    pub normal_background: Color,
    pub hovered_background: Color,
    pub pressed_background: Color,
    pub text_color: Color,
    pub border_color: Color,

    // Effects
    pub shadow_color: Color,
    pub shadow_offset: Vec2,
    pub shadow_blur: f32,

    // Transitions
    pub transition_duration: f32, // in seconds
}

// Style collection for all button variants
#[derive(Debug, Clone)]
pub struct ButtonVariantStyles {
    pub primary: ButtonStyle,
    pub secondary: ButtonStyle,
    pub destructive: ButtonStyle,
    pub outline: ButtonStyle,
    pub ghost: ButtonStyle,
    pub link: ButtonStyle,
}

impl ButtonVariantStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            primary: ButtonStyle {
                normal_background: colors.primary,
                hovered_background: colors.primary.with_alpha(0.9),
                pressed_background: colors.primary_foreground.with_alpha(0.8),
                text_color: colors.primary_foreground,
                border_color: Color::NONE,
                shadow_color: Color::NONE,
                shadow_offset: Vec2::ZERO,
                shadow_blur: 0.0,
                transition_duration: 0.2,
            },
            secondary: ButtonStyle {
                normal_background: colors.secondary,
                hovered_background: colors.secondary.with_alpha(0.8),
                pressed_background: colors.secondary.with_alpha(0.7),
                text_color: colors.secondary_foreground,
                border_color: Color::NONE,
                shadow_color: Color::NONE,
                shadow_offset: Vec2::ZERO,
                shadow_blur: 0.0,
                transition_duration: 0.2,
            },
            destructive: ButtonStyle {
                normal_background: colors.destructive,
                hovered_background: colors.destructive.with_alpha(0.9),
                pressed_background: colors.destructive.with_alpha(0.8),
                text_color: Color::WHITE,
                border_color: Color::NONE,
                shadow_color: Color::NONE,
                shadow_offset: Vec2::ZERO,
                shadow_blur: 0.0,
                transition_duration: 0.2,
            },
            outline: ButtonStyle {
                normal_background: colors.background,
                hovered_background: colors.accent,
                pressed_background: colors.accent.with_alpha(0.8),
                text_color: colors.foreground,
                border_color: colors.border,
                shadow_color: Color::NONE,
                shadow_offset: Vec2::ZERO,
                shadow_blur: 0.0,
                transition_duration: 0.2,
            },
            ghost: ButtonStyle {
                normal_background: Color::NONE,
                hovered_background: colors.accent,
                pressed_background: colors.accent.with_alpha(0.7),
                text_color: colors.accent_foreground,
                border_color: Color::NONE,
                shadow_color: Color::NONE,
                shadow_offset: Vec2::ZERO,
                shadow_blur: 0.0,
                transition_duration: 0.2,
            },
            link: ButtonStyle {
                normal_background: Color::NONE, // Link buttons typically have no background
                hovered_background: colors.accent.with_alpha(0.7),
                pressed_background: colors.accent.with_alpha(0.5),
                text_color: colors.accent_foreground,
                border_color: Color::NONE,
                shadow_color: Color::NONE,
                shadow_offset: Vec2::ZERO,
                shadow_blur: 0.0,
                transition_duration: 0.2,
            },
        }
    }
}

// Button size properties
#[derive(Debug, Clone)]
pub struct ButtonSizeProperties {
    pub font_size: f32,
    pub icon_size: f32,
    pub min_width: f32,
    pub min_height: f32,
    pub border_width: f32,
    pub border_radius: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
}

// Collection of size variants
#[derive(Debug, Clone)]
pub struct ButtonSizeStyles {
    pub xsmall: ButtonSizeProperties,
    pub small: ButtonSizeProperties,
    pub medium: ButtonSizeProperties,
    pub large: ButtonSizeProperties,
    pub xlarge: ButtonSizeProperties,
}

pub fn button_sizes() -> ButtonSizeStyles {
    ButtonSizeStyles {
        xsmall: ButtonSizeProperties {
            padding_horizontal: 6.0,
            padding_vertical: 2.0,
            font_size: 12.0,
            icon_size: 12.0,
            min_width: 56.0,
            min_height: 28.0,
            border_width: 1.0,
            border_radius: 4.0,
        },
        small: ButtonSizeProperties {
            padding_horizontal: 12.0,
            padding_vertical: 4.0,
            font_size: 14.0,
            icon_size: 14.0,
            min_width: 80.0,
            min_height: 32.0,
            border_width: 1.0,
            border_radius: 5.0,
        },
        medium: ButtonSizeProperties {
            padding_horizontal: 16.0,
            padding_vertical: 8.0,
            font_size: 15.0,
            icon_size: 16.0,
            min_width: 100.0,
            min_height: 36.0,
            border_width: 1.5,
            border_radius: 6.0,
        },
        large: ButtonSizeProperties {
            padding_horizontal: 24.0,
            padding_vertical: 8.0,
            font_size: 16.0,
            icon_size: 18.0,
            min_width: 120.0,
            min_height: 40.0,
            border_width: 2.0,
            border_radius: 6.0,
        },
        xlarge: ButtonSizeProperties {
            padding_horizontal: 28.0,
            padding_vertical: 10.0,
            font_size: 18.0,
            icon_size: 20.0,
            min_width: 144.0,
            min_height: 52.0,
            border_width: 2.0,
            border_radius: 8.0,
        },
    }
}
