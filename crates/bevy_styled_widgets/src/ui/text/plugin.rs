use bevy::prelude::*;

use super::update_text_styles;

pub struct StyledTextPlugin;
impl Plugin for StyledTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_text_styles);
    }
}
