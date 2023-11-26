use bevy::prelude::*;

use crate::{game::GameState, AppState};

pub fn log_system(app_state: Res<State<AppState>>, game_state: Res<State<GameState>>) {
	if app_state.is_changed() {
		info!("AppState: {:?}", app_state.get());
	}
	if game_state.is_changed() {
		info!("GameState: {:?}", game_state.get());
	}
}
