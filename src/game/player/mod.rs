use bevy::prelude::*;
use eat::player_eat;

use crate::{AppState, GameState};

use systems::{player_camera, player_jump, player_movement, player_spawn};

pub mod components;
pub mod eat;
pub mod nutrition;
pub mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::Game), player_spawn)
			.add_systems(
				Update,
				(player_camera, player_movement, player_jump, player_eat)
					.run_if(in_state(GameState::Playing)),
			);
	}
}
