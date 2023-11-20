use bevy::prelude::*;

use crate::game::GameState;

use super::AppState;

pub fn transition_to_game_state(
	k_input: Res<Input<KeyCode>>,
	mut n_app_state: ResMut<NextState<AppState>>,
	c_app_state: Res<State<AppState>>,
) {
	if k_input.just_pressed(KeyCode::G) {
		if **c_app_state != AppState::Game {
			n_app_state.set(AppState::Game);
			info!("Entered AppState::Game");
		}
	}
}

pub fn transition_to_main_menu_state(
	k_input: Res<Input<KeyCode>>,
	mut n_app_state: ResMut<NextState<AppState>>,
	c_app_state: Res<State<AppState>>,
) {
	if k_input.just_pressed(KeyCode::M) {
		if **c_app_state != AppState::MainMenu {
			n_app_state.set(AppState::MainMenu);
			info!("Entered AppState::MainMenu");
		}
	}
}

pub fn enter_game(mut game_state: ResMut<NextState<GameState>>) {
	game_state.set(GameState::Playing);
}

pub fn quit_game(mut game_state: ResMut<NextState<GameState>>) {
	game_state.set(GameState::None);
}
