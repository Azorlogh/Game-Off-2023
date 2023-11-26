use bevy::app::AppExit;
use bevy::prelude::*;

use crate::main_menu::styles::{BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::main_menu::MenuState;
use crate::AppState;

use super::{Options, Play, Quit};

pub fn interact_play_button(
	mut q_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Play>)>,
	mut app_state: ResMut<NextState<AppState>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				app_state.set(AppState::Game);
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

pub fn interact_option_button(
	mut q_button: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Options>),
	>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				menu_state.set(MenuState::Options)
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

pub fn interact_quit_button(
	mut q_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Quit>)>,
	mut app_exit_events: ResMut<Events<AppExit>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				app_exit_events.send(AppExit);
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
