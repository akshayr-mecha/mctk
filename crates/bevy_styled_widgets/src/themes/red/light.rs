#![allow(dead_code)]
use crate::themes::ThemeColors;
use bevy::prelude::*;

pub const BACKGROUND: Color = Color::oklch(1.0, 0.0, 0.0);
pub const FOREGROUND: Color = Color::oklch(0.141, 0.005, 285.823);
pub const CARD: Color = BACKGROUND;
pub const CARD_FOREGROUND: Color = FOREGROUND;
pub const POPOVER: Color = BACKGROUND;
pub const POPOVER_FOREGROUND: Color = FOREGROUND;
pub const PRIMARY: Color = Color::oklch(0.637, 0.237, 25.331);
pub const PRIMARY_FOREGROUND: Color = Color::oklch(0.971, 0.013, 17.38);
pub const SECONDARY: Color = Color::oklch(0.967, 0.001, 286.375);
pub const SECONDARY_FOREGROUND: Color = Color::oklch(0.21, 0.006, 285.885);
pub const MUTED: Color = SECONDARY;
pub const MUTED_FOREGROUND: Color = Color::oklch(0.552, 0.016, 285.938);
pub const ACCENT: Color = SECONDARY;
pub const ACCENT_FOREGROUND: Color = SECONDARY_FOREGROUND;
pub const DESTRUCTIVE: Color = Color::oklch(0.577, 0.245, 27.325);
pub const BORDER: Color = Color::oklch(0.92, 0.004, 286.32);
pub const INPUT: Color = BORDER;
pub const RING: Color = PRIMARY;
pub const CHART_1: Color = Color::oklch(0.646, 0.222, 41.116);
pub const CHART_2: Color = Color::oklch(0.6, 0.118, 184.704);
pub const CHART_3: Color = Color::oklch(0.398, 0.07, 227.392);
pub const CHART_4: Color = Color::oklch(0.828, 0.189, 84.429);
pub const CHART_5: Color = Color::oklch(0.769, 0.188, 70.08);
pub const SIDEBAR: Color = Color::oklch(0.985, 0.0, 0.0);
pub const SIDEBAR_FOREGROUND: Color = FOREGROUND;
pub const SIDEBAR_PRIMARY: Color = PRIMARY;
pub const SIDEBAR_PRIMARY_FOREGROUND: Color = PRIMARY_FOREGROUND;
pub const SIDEBAR_ACCENT: Color = SECONDARY;
pub const SIDEBAR_ACCENT_FOREGROUND: Color = SECONDARY_FOREGROUND;
pub const SIDEBAR_BORDER: Color = BORDER;
pub const SIDEBAR_RING: Color = RING;

pub const LIGHT_THEME_COLORS: ThemeColors = ThemeColors {
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
