use bevy::prelude::*;

use crate::AppState;

use self::systems::spawn_level;

pub(crate) mod systems;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		// Once the assets are loaded, spawn the level
		app.add_systems(OnEnter(AppState::Game), spawn_level);
	}
}
