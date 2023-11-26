use bevy::prelude::*;
use systems::log_system;
mod shadows;
mod systems;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (log_system, shadows::toggle_shadows));
	}
}