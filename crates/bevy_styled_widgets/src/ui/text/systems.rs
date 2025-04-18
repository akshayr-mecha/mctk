use bevy::{prelude::*, text::FontWeight};

use crate::{
    themes::{ThemeManager, text::TextStyle},
    ui::text::TextVariant,
};

use super::StyledText;

pub fn update_text_styles(
    theme_manager: Res<ThemeManager>,
    mut query: Query<(&mut TextFont, &mut TextColor, &mut TextLayout, &StyledText)>,
) {
    let theme_styles = &theme_manager.styles;

    for (mut text_font, mut text_color, mut text_layout, styled_text) in query.iter_mut() {
        let mut style = match styled_text.variant {
            TextVariant::Header1 => TextStyle {
                font_size: 32.0,
                font_weight: FontWeight::BOLD,
                color: theme_styles.text.color,
                ..Default::default()
            },
            TextVariant::Header2 => TextStyle {
                font_size: 24.0,
                font_weight: FontWeight::BOLD,
                color: theme_styles.text.color,
                ..Default::default()
            },
            TextVariant::Header3 => TextStyle {
                font_size: 20.0,
                font_weight: FontWeight::SEMIBOLD,
                color: theme_styles.text.color,
                ..Default::default()
            },
            TextVariant::Body => TextStyle {
                font_size: 16.0,
                font_weight: FontWeight::NORMAL,
                color: theme_styles.text.color,
                ..Default::default()
            },
            TextVariant::Caption => TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::NORMAL,
                color: theme_styles.text.color.with_alpha(0.8),
                ..Default::default()
            },
            TextVariant::Small => TextStyle {
                font_size: 12.0,
                font_weight: FontWeight::NORMAL,
                color: theme_styles.text.color.with_alpha(0.7),
                ..Default::default()
            },
            TextVariant::Quote => TextStyle {
                font_size: 18.0,
                font_weight: FontWeight::LIGHT,
                color: theme_styles.text.color.with_alpha(0.9),
                ..Default::default()
            },
            TextVariant::Code => TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::NORMAL,
                color: theme_styles.text.color,
                ..Default::default()
            },
            TextVariant::Label => TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::MEDIUM,
                color: theme_styles.text.color,
                ..Default::default()
            },
            TextVariant::Error => TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::NORMAL,
                color: Color::srgb(0.9, 0.2, 0.2),
                ..Default::default()
            },
            TextVariant::Success => TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::NORMAL,
                color: Color::srgb(0.2, 0.8, 0.2),
                ..Default::default()
            },
            TextVariant::Warning => TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::NORMAL,
                color: Color::srgb(0.9, 0.7, 0.0),
                ..Default::default()
            },
            TextVariant::Info => TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::NORMAL,
                color: Color::srgb(0.2, 0.6, 0.9),
                ..Default::default()
            },
            TextVariant::Custom => theme_styles.text.clone(),
        };

        // Override with custom properties if specified
        if let Some(color) = styled_text.color {
            style.color = color;
        }

        if let Some(font_size) = styled_text.font_size {
            style.font_size = font_size;
        }
        if let Some(weight) = styled_text.font_weight {
            style.font_weight = weight;
        }

        if let Some(line_height) = styled_text.line_height {
            style.line_height = line_height;
        }

        if let Some(alignment) = styled_text.alignment {
            style.text_align = alignment;
        }

        // Apply the style to the text component
        text_font.font_size = style.font_size;
        text_color.0 = style.color;
        text_layout.justify = style.text_align;
    }
}
