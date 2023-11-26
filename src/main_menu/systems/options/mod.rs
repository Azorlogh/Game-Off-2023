use bevy::prelude::*;

pub mod interactions;
pub mod layout;

use interactions::*;
use layout::*;

use crate::main_menu::MenuState;

// Options Menu
#[derive(Component)]
pub struct OptionsMenu {}

#[derive(Component)]
pub struct OptionsBack {}

#[derive(Component)]
pub struct Keyboard {}

#[derive(Component)]
pub struct Sounds {}

#[derive(Component)]
pub struct Graphics {}

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				interact_back_button,
				interact_sounds_button,
				interact_keyboard_button,
				interact_graphics_button,
			),
		)
		.add_systems(OnEnter(MenuState::Options), spawn_option_menu)
		.add_systems(OnExit(MenuState::Options), despawn_option_menu);
	}
}
