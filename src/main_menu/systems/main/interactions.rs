use bevy::app::AppExit;
use bevy::prelude::*;

use crate::main_menu::MenuState;
use crate::AppState;

use super::{Options, Play, Quit};

pub fn interact_play_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Play>)>,
	mut app_state: ResMut<NextState<AppState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				app_state.set(AppState::Game);
			}
			_ => {}
		}
	}
}

pub fn interact_option_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Options>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => menu_state.set(MenuState::Options),
			_ => {}
		}
	}
}

pub fn interact_quit_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Quit>)>,
	mut app_exit_events: ResMut<Events<AppExit>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				app_exit_events.send(AppExit);
			}
			_ => {}
		}
	}
}
