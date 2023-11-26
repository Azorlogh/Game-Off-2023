use bevy::prelude::*;

use super::{food::components::Food, DespawnOnExitGame, GameState};

// TODO: Mettre les bonnes touches
pub fn toggle_game(
	mut commands: Commands,
	keyboard_input: Res<Input<KeyCode>>,
	game_state: Res<State<GameState>>,
) {
	if keyboard_input.just_pressed(KeyCode::P) {
		println!("Game State: {:?}", **game_state);
		if **game_state == GameState::Playing {
			commands.insert_resource(NextState(Some(GameState::Pause)));
			info!("Game Paused !");
		}

		if **game_state == GameState::Pause {
			commands.insert_resource(NextState(Some(GameState::Playing)));
			info!("Game Running !");
		}
	}
}

pub fn despawn_game(
	mut commands: Commands,
	q_game_despawn: Query<Entity, With<DespawnOnExitGame>>,
	q_food_despawn: Query<Entity, With<Food>>,
) {
	for game_entity in q_game_despawn.iter() {
		commands.entity(game_entity).despawn_recursive();
	}

	for food_entity in q_food_despawn.iter() {
		commands.entity(food_entity).despawn_recursive();
	}
}
