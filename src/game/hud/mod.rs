use bevy::prelude::*;

mod crosshair;
pub(crate) mod player_ui;

use player_ui::*;

use self::crosshair::CrosshairPlugin;

pub struct HudPlugin;
impl Plugin for HudPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((CrosshairPlugin, PlayerUiPlugin));
	}
}
