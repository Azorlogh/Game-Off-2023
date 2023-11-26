use bevy::prelude::*;

use crate::main_menu::MenuState;

use super::GraphicsBack;

pub fn interact_back_button(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<GraphicsBack>)>,
	mut menu_state: ResMut<NextState<MenuState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => menu_state.set(MenuState::Options),
			_ => {}
		}
	}
}
