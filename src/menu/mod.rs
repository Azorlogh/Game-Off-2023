use bevy::prelude::*;

use crate::game::GameState;

pub(crate) use components::{MenuState, OptionState};
mod components;

use systems::{ui_options, ui_pause_game, ui_system};
mod systems;

// TODO: Refactor avec MainMenu
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<MenuState>()
			.add_state::<OptionState>()
			.add_systems(
				Update,
				(
					ui_system.run_if(in_state(GameState::Menu).and_then(in_state(MenuState::Menu))),
					ui_pause_game
						.run_if(in_state(GameState::Pause).and_then(in_state(MenuState::Menu))),
					ui_options.run_if(in_state(MenuState::Option)),
				),
			);
	}
}
