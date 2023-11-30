use bevy::prelude::*;

use crate::game::GameState;

pub fn enter_game(mut game_state: ResMut<NextState<GameState>>) {
	game_state.set(GameState::Playing);
}

pub fn quit_game(mut game_state: ResMut<NextState<GameState>>) {
	game_state.set(GameState::None);
}
