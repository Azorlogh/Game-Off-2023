use bevy::prelude::*;

use super::{ActionButton, KeyText, KeyboardBack};
use crate::{
	menu::MenuState,
	settings::{Action, GeneralInput, Settings},
};

#[derive(Resource)]
pub struct ButtonState(pub Option<Action>);

pub fn interact_action_button(
	mut q_button: Query<(&Interaction, &ActionButton), Changed<Interaction>>,
	mut settings: ResMut<Settings>,
	mut button_state: ResMut<ButtonState>,

	keys: Res<Input<KeyCode>>,
	buttons: Res<Input<MouseButton>>,
) {
	match &mut button_state.0 {
		Some(movement) => {
			let mut general_input = None;

			for k in keys.get_just_pressed() {
				general_input = Some(GeneralInput::KeyCode(*k));
			}

			for b in buttons.get_just_pressed() {
				general_input = Some(GeneralInput::MouseButton(*b));
			}

			if let Some(input) = general_input {
				*settings.input.get_mut(&movement).unwrap() = input;
				button_state.0 = None;
			}
		}
		_ => {
			for (interaction, action) in &mut q_button {
				match *interaction {
					Interaction::Pressed => {
						// change user action
						button_state.0 = Some(action.0);
					}
					_ => {}
				}
			}
		}
	}
}

pub fn update_button_text(
	q_action_button: Query<&ActionButton>,
	mut q_text: Query<(&mut Text, &Parent), With<KeyText>>,
	button_state: Res<ButtonState>,
	settings: ResMut<Settings>,
) {
	for (mut text, parent) in &mut q_text {
		let action_btn = q_action_button.get(parent.get()).unwrap();
		match button_state.0 {
			Some(action) if action_btn.0 == action => {
				text.sections[0].value = String::from("???");
			}
			_ => {
				let key_button = *settings.input.get(&action_btn.0).unwrap();
				text.sections[0].value = format!("{}", key_button);

				text.sections[0].style.color = Color::Rgba {
					red: 1.0,
					green: 0.0,
					blue: 1.0,
					alpha: 1.0,
				}
			}
		}
	}
}

pub fn interact_back_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<KeyboardBack>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => menu_state.set(MenuState::Options),
			_ => {}
		}
	}
}
