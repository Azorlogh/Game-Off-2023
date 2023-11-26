use bevy::prelude::*;

use crate::main_menu::styles::{BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::main_menu::MenuState;

use super::{Graphics, Keyboard, OptionsBack, Sounds};

pub fn interact_sounds_button(
	mut q_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Sounds>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				// menu_state.set(MenuState::Sounds)
			}
			Interaction::Hovered => {
				*background_color = HOVERED_BUTTON_COLOR.into();
			}
			Interaction::None => {
				*background_color = BUTTON_COLOR.into();
			}
		}
	}
}

pub fn interact_keyboard_button(
	mut q_button: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Keyboard>),
	>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				menu_state.set(MenuState::Keyboard)
			}
			Interaction::Hovered => {
				*background_color = HOVERED_BUTTON_COLOR.into();
			}
			Interaction::None => {
				*background_color = BUTTON_COLOR.into();
			}
		}
	}
}

pub fn interact_graphics_button(
	mut q_button: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Graphics>),
	>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				// menu_state.set(MenuState::Graphics)
			}
			Interaction::Hovered => {
				*background_color = HOVERED_BUTTON_COLOR.into();
			}
			Interaction::None => {
				*background_color = BUTTON_COLOR.into();
			}
		}
	}
}

pub fn interact_back_button(
	mut q_button: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<OptionsBack>),
	>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				menu_state.set(MenuState::Main)
			}
			Interaction::Hovered => {
				*background_color = HOVERED_BUTTON_COLOR.into();
			}
			Interaction::None => {
				*background_color = BUTTON_COLOR.into();
			}
		}
	}
}
