use bevy::prelude::*;

pub(crate) mod health;
pub(crate) mod player_ui;

use health::*;
use player_ui::*;

pub struct HudPlugin;
impl Plugin for HudPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((HealthPlugin, PlayerUiPlugin));
	}
}
