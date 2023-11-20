use bevy::prelude::*;

use crate::{game::GameState, AppState};

pub fn print_states(app_state: Res<State<AppState>>, game_state: Res<State<GameState>>) {
	if app_state.is_changed() {
		debug!("AppState: {:?}", app_state.get());
	}
	if game_state.is_changed() {
		debug!("GameState: {:?}", game_state.get());
	}
}
