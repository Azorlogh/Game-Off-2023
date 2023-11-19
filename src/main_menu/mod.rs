mod components;
mod styles;
mod systems;

use crate::GameState;
use bevy::prelude::*;
use systems::layout::*;

use self::systems::interactions::{
	interact_option_button, interact_play_button, interact_quit_button,
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
			.add_systems(
				Update,
				(
					interact_play_button,
					interact_option_button,
					interact_quit_button,
				),
			)
			.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
	}
}
