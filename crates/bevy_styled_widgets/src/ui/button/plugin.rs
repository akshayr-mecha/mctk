use super::systems::{init, update_button_bg_colors, update_text_color};
use bevy::prelude::*;

pub struct StyledButtonPlugin;
impl Plugin for StyledButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init);
        app.add_systems(Update, (update_button_bg_colors, update_text_color));
    }
}
