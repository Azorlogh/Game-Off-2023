use bevy::prelude::*;

use crate::main_menu::MenuState;

use super::{Graphics, Keyboard, OptionsBack, Sounds};

pub fn interact_sounds_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Sounds>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				// menu_state.set(MenuState::Sounds)
			}
			_ => {}
		}
	}
}

pub fn interact_keyboard_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Keyboard>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => menu_state.set(MenuState::Keyboard),
			_ => {}
		}
	}
}

pub fn interact_graphics_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Graphics>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				// menu_state.set(MenuState::Graphics)
			}
			_ => {}
		}
	}
}

pub fn interact_back_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<OptionsBack>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => menu_state.set(MenuState::Main),
			_ => {}
		}
	}
}
