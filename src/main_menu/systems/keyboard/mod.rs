use bevy::prelude::*;

pub mod interactions;
pub mod layout;

use interactions::*;
use layout::*;

use crate::main_menu::MenuState;

// Keyboard Menu
#[derive(Component)]
pub struct KeyboardMenu {}

#[derive(Component)]
pub struct KeyboardBack {}

#[derive(Component)]
pub struct Forward {}

#[derive(Component)]
pub struct Backward {}

#[derive(Component)]
pub struct Left {}

#[derive(Component)]
pub struct Right {}

#[derive(Component)]
pub struct Eat {}

#[derive(Component)]
pub struct Jump {}

pub struct KeyboardMenuPlugin;
impl Plugin for KeyboardMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				interact_back_button,
				interact_forward_button,
				// interact_backward_button,
				// interact_left_button,
				// interact_right_button,
				// interact_eat_button,
				// interact_jump_button,
			),
		)
		.add_systems(OnEnter(MenuState::Keyboard), spawn_menu)
		.add_systems(OnExit(MenuState::Keyboard), despawn_menu);
	}
}
