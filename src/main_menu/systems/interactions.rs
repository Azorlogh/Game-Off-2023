use bevy::app::AppExit;
use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::{BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::GameState;

pub fn interact_play_button(
	mut q_button: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<MainMenuPlay>),
	>,
	mut app_state: ResMut<NextState<GameState>>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
				app_state.set(GameState::Running);
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

// TODO options dans le MainMenu
pub fn interact_option_button(
	mut q_button: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<MainMenuOptions>),
	>,
) {
	if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				*background_color = PRESSED_BUTTON_COLOR.into();
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
	mut q_button: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<MainMenuQuit>),
	>,
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
