use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{CoreButton, hover::Hovering};

use super::{
    ButtonSize,
    components::{AccessibleName, ButtonVariant, StyledButton, StyledButtonText},
};

#[derive(Default)]
pub struct ButtonBuilder {
    variant: ButtonVariant,
    on_click: Option<SystemId>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    hover_background_color: Option<Color>,
    hover_border_color: Option<Color>,
    text_color: Option<Color>,
    text: Option<String>,
    icon: Option<String>,
    size: Option<ButtonSize>,
    disabled: bool,
}

impl ButtonBuilder {
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn on_click(mut self, system_id: SystemId) -> Self {
        self.on_click = Some(system_id);
        self
    }

    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn hover_background_color(mut self, color: Color) -> Self {
        self.hover_background_color = Some(color);
        self
    }

    pub fn hover_border_color(mut self, color: Color) -> Self {
        self.hover_border_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn build(self) -> impl Bundle {
        (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            BorderRadius::default(),
            BorderColor::default(),
            Name::new("Button"),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            StyledButton {
                text: self.text.clone(),
                icon: self.icon.clone(),
                variant: self.variant,
                size: self.size,
                on_click: self.on_click,
                background_color: self.background_color,
                border_color: self.border_color,
                hover_background_color: self.hover_background_color,
                hover_border_color: self.hover_border_color,
                text_color: self.text_color,
                disabled: self.disabled,
            },
            CoreButton {
                on_click: self.on_click,
            },
            AccessibleName(self.text.clone().unwrap_or_else(|| "Button".to_string())),
            TabIndex(0),
            // children![(
            //     Text::new(self.text.clone().unwrap_or_else(|| "Button".to_string())),
            //     TextFont {
            //         font_size: 14.0,
            //         ..default()
            //     },
            //     StyledButtonText {
            //         variant: self.variant,
            //         size: self.size,
            //     }
            // )],
        )
    }
}
