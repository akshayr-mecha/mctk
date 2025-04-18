use bevy::{prelude::*, state::commands, text::LineHeight};
use bevy_core_widgets::{ButtonPressed, InteractionDisabled, hover::Hovering};

use crate::themes::ThemeManager;

use super::{
    ButtonSize,
    components::{ButtonVariant, StyledButton, StyledButtonText},
};

// Update text color
pub fn update_text_color(
    theme_manager: Res<ThemeManager>,
    mut query: Query<(&mut TextColor, &mut TextFont, &StyledButtonText)>,
) {
    for (mut text_color, mut text_font, button) in query.iter_mut() {
        let button_styles = theme_manager.styles.buttons.clone();
        let button_size_styles = theme_manager.styles.button_sizes.clone();
        let button_style = match button.variant {
            ButtonVariant::Primary => button_styles.primary,
            ButtonVariant::Secondary => button_styles.secondary,
            ButtonVariant::Destructive => button_styles.destructive,
            ButtonVariant::Outline => button_styles.outline,
            ButtonVariant::Ghost => button_styles.ghost,
        };
        let color = button_style.text_color;
        text_color.0 = color;

        //update font size
        let button_size_style = match button.size.unwrap_or_default() {
            ButtonSize::XSmall => button_size_styles.xsmall,
            ButtonSize::Small => button_size_styles.small,
            ButtonSize::Medium => button_size_styles.medium,
            ButtonSize::Large => button_size_styles.large,
            ButtonSize::XLarge => button_size_styles.xlarge,
        };
        text_font.font_size = button_size_style.font_size;
    }
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
pub fn update_button_bg_colors(
    theme_manager: Res<ThemeManager>,
    mut query: Query<(
        &mut Node,
        &StyledButton,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut BorderRadius,
        &Hovering,
        &ButtonPressed,
        Has<InteractionDisabled>,
    )>,
) {
    for (
        mut button_node,
        button,
        mut bg_color,
        mut border_color,
        mut border_radius,
        Hovering(is_hovering),
        ButtonPressed(is_pressed),
        is_disabled,
    ) in query.iter_mut()
    {
        // Get styles from theme manager
        let button_styles = theme_manager.styles.buttons.clone();
        let button_size_styles = theme_manager.styles.button_sizes.clone();

        // Update the background color based on the button's state
        let button_style = match button.variant {
            ButtonVariant::Primary => button_styles.primary,
            ButtonVariant::Secondary => button_styles.secondary,
            ButtonVariant::Destructive => button_styles.destructive,
            ButtonVariant::Outline => button_styles.outline,
            ButtonVariant::Ghost => button_styles.ghost,
        };

        match (is_disabled, is_pressed, is_hovering) {
            (true, _, _) => {
                bg_color.0 = button_style.normal_background;
                border_color.0 = button_style.border_color;
            }
            (_, true, true) => {
                bg_color.0 = button_style.pressed_background;
                border_color.0 = button_style.border_color;
            }
            (_, false, true) => {
                bg_color.0 = button_style.hovered_background;
                border_color.0 = button_style.border_color;
            }
            _ => {
                bg_color.0 = button_style.normal_background;
                border_color.0 = button_style.border_color;
            }
        };

        //Update size styles
        let button_size_style = match button.size.unwrap_or_default() {
            ButtonSize::XSmall => button_size_styles.xsmall,
            ButtonSize::Small => button_size_styles.small,
            ButtonSize::Medium => button_size_styles.medium,
            ButtonSize::Large => button_size_styles.large,
            ButtonSize::XLarge => button_size_styles.xlarge,
        };
        button_node.padding = UiRect::axes(
            Val::Px(button_size_style.padding_horizontal),
            Val::Px(button_size_style.padding_vertical),
        );
        button_node.border = UiRect::all(Val::Px(button_size_style.border_width));
        border_radius.top_left = Val::Px(button_size_style.border_radius);
        border_radius.top_right = Val::Px(button_size_style.border_radius);
        border_radius.bottom_left = Val::Px(button_size_style.border_radius);
        border_radius.bottom_right = Val::Px(button_size_style.border_radius);
    }
}

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &StyledButton)>,
) {
    println!("init()");
    for (entity, button) in query.iter_mut() {
        let mut command = commands.entity(entity);

        // if icon is present, add it as a child
        if let Some(icon) = button.icon.clone() {
            command.with_children(|parent| {
                parent.spawn((
                    ImageNode::new(asset_server.load(&icon)),
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        ..default()
                    },
                ));
            });
        }

        // if text button, add text as children
        if let Some(text) = button.text.clone() {
            command.with_children(|parent| {
                parent.spawn((
                    Text::new(text.clone()),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    StyledButtonText {
                        variant: button.variant,
                        size: button.size,
                    },
                ));
            });
        }
    }
}
