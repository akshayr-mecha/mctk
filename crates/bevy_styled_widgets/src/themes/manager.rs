use std::collections::HashMap;

use bevy::prelude::*;

use super::styles::ThemeStyles;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeId(pub String);

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    pub chart: Color,
    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_primary: Color,
    pub sidebar_primary_foreground: Color,
    pub sidebar_accent: Color,
    pub sidebar_accent_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_ring: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Theme {
    Default,
    Red,
    Rose,
    Orange,
    Green,
    Blue,
    Yellow,
    Violet,
}

impl Into<String> for Theme {
    fn into(self) -> String {
        match self {
            Theme::Default => "default".into(),
            Theme::Red => "red".to_string(),
            Theme::Rose => "rose".to_string(),
            Theme::Orange => "orange".to_string(),
            Theme::Green => "green".to_string(),
            Theme::Blue => "blue".to_string(),
            Theme::Yellow => "yellow".to_string(),
            Theme::Violet => "violet".to_string(),
        }
    }
}
impl Into<ThemeId> for Theme {
    fn into(self) -> ThemeId {
        ThemeId(self.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Light
    }
}

#[derive(Resource)]
pub struct ThemeManager {
    pub themes: HashMap<(ThemeId, ThemeMode), ThemeColors>,
    pub current_theme: ThemeId,
    pub current_mode: ThemeMode,
    pub styles: ThemeStyles,
}

impl Default for ThemeManager {
    fn default() -> Self {
        let mut themes = HashMap::new();
        themes.insert(
            (ThemeId(Theme::Default.into()), ThemeMode::Light),
            super::default::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Default.into()), ThemeMode::Dark),
            super::default::dark::DARK_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Red.into()), ThemeMode::Light),
            super::red::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Red.into()), ThemeMode::Dark),
            super::red::dark::DARK_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Rose.into()), ThemeMode::Light),
            super::rose::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Rose.into()), ThemeMode::Dark),
            super::rose::dark::DARK_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Orange.into()), ThemeMode::Light),
            super::orange::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Orange.into()), ThemeMode::Dark),
            super::orange::dark::DARK_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Green.into()), ThemeMode::Light),
            super::green::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Green.into()), ThemeMode::Dark),
            super::green::dark::DARK_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Blue.into()), ThemeMode::Light),
            super::blue::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Blue.into()), ThemeMode::Dark),
            super::blue::dark::DARK_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Yellow.into()), ThemeMode::Light),
            super::yellow::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Yellow.into()), ThemeMode::Dark),
            super::yellow::dark::DARK_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Violet.into()), ThemeMode::Light),
            super::violet::light::LIGHT_THEME_COLORS,
        );
        themes.insert(
            (ThemeId(Theme::Violet.into()), ThemeMode::Dark),
            super::violet::dark::DARK_THEME_COLORS,
        );

        Self {
            themes,
            current_theme: ThemeId(Theme::Default.into()),
            current_mode: ThemeMode::Light,
            styles: ThemeStyles::from_colors(super::default::light::LIGHT_THEME_COLORS),
        }
    }
}

impl ThemeManager {
    pub fn set_theme(&mut self, theme: ThemeId) {
        if let Some(colors) = self.themes.get(&(theme.clone(), self.current_mode.clone())) {
            self.current_theme = theme.clone();
            self.styles = ThemeStyles::from_colors(colors.clone());
        } else {
            warn!("Theme {:?} not found", theme);
        }
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        if let Some(colors) = self.themes.get(&(self.current_theme.clone(), mode.clone())) {
            self.current_mode = mode.clone();
            self.styles = ThemeStyles::from_colors(colors.clone());
        } else {
            warn!(
                "Theme {:?} with mode {:?} not found",
                self.current_theme, mode
            );
        }
    }
    pub fn set_theme_and_mode(&mut self, theme: ThemeId, mode: ThemeMode) {
        if let Some(colors) = self.themes.get(&(theme.clone(), mode.clone())) {
            self.current_theme = theme.clone();
            self.current_mode = mode.clone();
            self.styles = ThemeStyles::from_colors(colors.clone());
        } else {
            warn!("Theme {:?} with mode {:?} not found", theme, mode);
        }
    }

    pub fn add_theme(
        &mut self,
        theme: ThemeId,
        mode: ThemeMode,
        colors: ThemeColors,
    ) -> Result<(), String> {
        if self.themes.contains_key(&(theme.clone(), mode.clone())) {
            return Err(format!(
                "Theme {:?} with mode {:?} already exists",
                theme, mode
            ));
        }
        self.themes.insert((theme, mode), colors);
        Ok(())
    }

    pub fn remove_theme(&mut self, theme: ThemeId, mode: ThemeMode) -> Result<(), String> {
        if self.themes.remove(&(theme.clone(), mode.clone())).is_none() {
            return Err(format!("Theme {:?} with mode {:?} not found", theme, mode));
        }
        Ok(())
    }

    pub fn update_theme(
        &mut self,
        theme: ThemeId,
        mode: ThemeMode,
        colors: ThemeColors,
    ) -> Result<(), String> {
        if !self.themes.contains_key(&(theme.clone(), mode.clone())) {
            return Err(format!("Theme {:?} with mode {:?} not found", theme, mode));
        }
        self.themes.insert((theme, mode), colors);
        Ok(())
    }

    pub fn get_theme(&self, theme: ThemeId, mode: ThemeMode) -> Option<&ThemeColors> {
        self.themes.get(&(theme, mode))
    }
}
