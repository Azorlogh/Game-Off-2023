use bevy::prelude::*;

pub mod interactions;
pub mod layout;

use interactions::*;
use layout::*;

use crate::main_menu::MenuState;

// Main Menu
#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct Options;

#[derive(Component)]
pub struct Play;

#[derive(Component)]
pub struct Quit;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				interact_play_button,
				interact_option_button,
				interact_quit_button,
			),
		)
		.add_systems(OnEnter(MenuState::Main), spawn_main_menu)
		.add_systems(OnExit(MenuState::Main), despawn_main_menu);
	}
}
