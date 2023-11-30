use bevy::prelude::*;

pub mod interactions;
pub mod layout;

use interactions::*;
use layout::*;

use crate::{main_menu::MenuState, settings::Action};

// Keyboard Menu
#[derive(Component)]
pub struct KeyboardMenu;

#[derive(Component)]
pub struct KeyboardBack;

#[derive(Component)]
pub struct ActionButton(Action);

#[derive(Component)]
pub struct KeyText;

pub struct KeyboardMenuPlugin;
impl Plugin for KeyboardMenuPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ButtonState(None))
			.add_systems(
				Update,
				(
					interact_back_button,
					interact_action_button,
					update_button_text,
					// interact_forward_button,
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
