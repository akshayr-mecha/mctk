#![allow(dead_code)]
use bevy::prelude::*;

use crate::themes::ThemeColors;

pub const BACKGROUND: Color = Color::oklch(1.0, 0.0, 0.0);
pub const FOREGROUND: Color = Color::oklch(0.145, 0.0, 0.0);
pub const CARD: Color = Color::oklch(1.0, 0.0, 0.0);
pub const CARD_FOREGROUND: Color = Color::oklch(0.145, 0.0, 0.0);
pub const POPOVER: Color = Color::oklch(1.0, 0.0, 0.0);
pub const POPOVER_FOREGROUND: Color = Color::oklch(0.145, 0.0, 0.0);
pub const PRIMARY: Color = Color::oklch(0.205, 0.0, 0.0);
pub const PRIMARY_FOREGROUND: Color = Color::oklch(0.985, 0.0, 0.0);
pub const SECONDARY: Color = Color::oklch(0.97, 0.0, 0.0);
pub const SECONDARY_FOREGROUND: Color = Color::oklch(0.205, 0.0, 0.0);
pub const MUTED: Color = Color::oklch(0.97, 0.0, 0.0);
pub const MUTED_FOREGROUND: Color = Color::oklch(0.556, 0.0, 0.0);
pub const ACCENT: Color = Color::oklch(0.97, 0.0, 0.0);
pub const ACCENT_FOREGROUND: Color = Color::oklch(0.205, 0.0, 0.0);
pub const DESTRUCTIVE: Color = Color::oklch(0.577, 0.245, 27.325);
pub const BORDER: Color = Color::oklch(0.922, 0.0, 0.0);
pub const INPUT: Color = Color::oklch(0.922, 0.0, 0.0);
pub const RING: Color = Color::oklch(0.708, 0.0, 0.0);
pub const CHART_1: Color = Color::oklch(0.646, 0.222, 41.116);
pub const CHART_2: Color = Color::oklch(0.6, 0.118, 184.704);
pub const CHART_3: Color = Color::oklch(0.398, 0.07, 227.392);
pub const CHART_4: Color = Color::oklch(0.828, 0.189, 84.429);
pub const CHART_5: Color = Color::oklch(0.769, 0.188, 70.08);
pub const SIDEBAR: Color = Color::oklch(0.985, 0.0, 0.0);
pub const SIDEBAR_FOREGROUND: Color = Color::oklch(0.145, 0.0, 0.0);
pub const SIDEBAR_PRIMARY: Color = Color::oklch(0.205, 0.0, 0.0);
pub const SIDEBAR_PRIMARY_FOREGROUND: Color = Color::oklch(0.985, 0.0, 0.0);
pub const SIDEBAR_ACCENT: Color = Color::oklch(0.97, 0.0, 0.0);
pub const SIDEBAR_ACCENT_FOREGROUND: Color = Color::oklch(0.205, 0.0, 0.0);
pub const SIDEBAR_BORDER: Color = Color::oklch(0.922, 0.0, 0.0);
pub const SIDEBAR_RING: Color = Color::oklch(0.708, 0.0, 0.0);

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
