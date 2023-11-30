use bevy::prelude::*;
use systems::log_system;

mod end_game;
mod lighting;
mod physics;
mod scale;
mod systems;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				log_system,
				lighting::toggle_shadows,
				lighting::despawn_lights,
				scale::scale,
				physics::toggle_debug,
				end_game::trigger_end,
			),
		);
	}
}
