//! dark.rs
#![allow(dead_code)]
use crate::themes::ThemeColors;
use bevy::prelude::*;

pub const BACKGROUND: Color = Color::oklch(0.141, 0.005, 285.823);
pub const FOREGROUND: Color = Color::oklch(0.985, 0.0, 0.0);
pub const CARD: Color = Color::oklch(0.21, 0.006, 285.885);
pub const CARD_FOREGROUND: Color = FOREGROUND;
pub const POPOVER: Color = CARD;
pub const POPOVER_FOREGROUND: Color = FOREGROUND;
pub const PRIMARY: Color = Color::oklch(0.645, 0.246, 16.439);
pub const PRIMARY_FOREGROUND: Color = Color::oklch(0.969, 0.015, 12.422);
pub const SECONDARY: Color = Color::oklch(0.274, 0.006, 286.033);
pub const SECONDARY_FOREGROUND: Color = FOREGROUND;
pub const MUTED: Color = SECONDARY;
pub const MUTED_FOREGROUND: Color = Color::oklch(0.705, 0.015, 286.067);
pub const ACCENT: Color = SECONDARY;
pub const ACCENT_FOREGROUND: Color = FOREGROUND;
pub const DESTRUCTIVE: Color = Color::oklch(0.704, 0.191, 22.216);
pub const BORDER: Color = Color::oklch(1.0, 0.0, 0.0); // originally with alpha / 10%
pub const INPUT: Color = Color::oklch(1.0, 0.0, 0.0); // originally with alpha / 15%
pub const RING: Color = PRIMARY;
pub const CHART_1: Color = Color::oklch(0.488, 0.243, 264.376);
pub const CHART_2: Color = Color::oklch(0.696, 0.17, 162.48);
pub const CHART_3: Color = Color::oklch(0.769, 0.188, 70.08);
pub const CHART_4: Color = Color::oklch(0.627, 0.265, 303.9);
pub const CHART_5: Color = PRIMARY;
pub const SIDEBAR: Color = CARD;
pub const SIDEBAR_FOREGROUND: Color = FOREGROUND;
pub const SIDEBAR_PRIMARY: Color = PRIMARY;
pub const SIDEBAR_PRIMARY_FOREGROUND: Color = PRIMARY_FOREGROUND;
pub const SIDEBAR_ACCENT: Color = SECONDARY;
pub const SIDEBAR_ACCENT_FOREGROUND: Color = FOREGROUND;
pub const SIDEBAR_BORDER: Color = BORDER;
pub const SIDEBAR_RING: Color = RING;

pub const DARK_THEME_COLORS: ThemeColors = ThemeColors {
    background: BACKGROUND,
    foreground: FOREGROUND,
    card: CARD,
    card_foreground: CARD_FOREGROUND,
    popover: POPOVER,
    popover_foreground: POPOVER_FOREGROUND,
    primary: PRIMARY,
    primary_foreground: PRIMARY_FOREGROUND,
    secondary: SECONDARY,
    secondary_foreground: SECONDARY_FOREGROUND,
    muted: MUTED,
    muted_foreground: MUTED_FOREGROUND,
    accent: ACCENT,
    accent_foreground: ACCENT_FOREGROUND,
    destructive: DESTRUCTIVE,
    border: BORDER,
    input: INPUT,
    ring: RING,
    chart: CHART_1,
    sidebar: SIDEBAR,
    sidebar_foreground: SIDEBAR_FOREGROUND,
    sidebar_primary: SIDEBAR_PRIMARY,
    sidebar_primary_foreground: SIDEBAR_PRIMARY_FOREGROUND,
    sidebar_accent: SIDEBAR_ACCENT,
    sidebar_accent_foreground: SIDEBAR_ACCENT_FOREGROUND,
    sidebar_border: SIDEBAR_BORDER,
    sidebar_ring: SIDEBAR_RING,
};
