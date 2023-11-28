use bevy::prelude::*;

pub(crate) mod player_ui;

use player_ui::*;

pub struct HudPlugin;
impl Plugin for HudPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PlayerUiPlugin);
	}
}
