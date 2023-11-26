pub(crate) mod components;
mod styles;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::layout::*;

use self::systems::interactions::{
	interact_back_button, interact_option_button, interact_play_button, interact_quit_button,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MenuState {
	#[default]
	Root,
	Options,
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<MenuState>()
			.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
			.add_systems(
				Update,
				(
					interact_play_button,
					interact_option_button,
					interact_quit_button,
					interact_back_button,
				),
			)
			.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
	}
}
