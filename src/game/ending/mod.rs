use bevy::prelude::*;

use self::{lose_state::LosePlugin, win_state::WinPlugin};
use super::GameState;

mod lose_state;
mod win_state;

pub struct GameEndPlugin;
impl Plugin for GameEndPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((WinPlugin, LosePlugin))
			.add_event::<Win>()
			.add_event::<Lose>()
			.add_systems(Update, (handle_win, handle_lose));
	}
}

#[derive(Event)]
pub struct Win;

#[derive(Event)]
pub struct Lose;

fn handle_win(mut next_state: ResMut<NextState<GameState>>, mut ev_win: EventReader<Win>) {
	if ev_win.iter().count() > 0 {
		next_state.set(GameState::Win);
	}
}

fn handle_lose(mut next_state: ResMut<NextState<GameState>>, mut ev_lose: EventReader<Lose>) {
	if ev_lose.iter().count() > 0 {
		next_state.set(GameState::Lose);
	}
}
