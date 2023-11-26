use bevy::prelude::*;

pub mod main;
use main::MainMenuPlugin;

pub mod options;
use options::OptionsMenuPlugin;

pub mod keyboard;
use keyboard::KeyboardMenuPlugin;

use crate::main_menu::components::MainMenuCamera;
use crate::main_menu::systems::main::MainMenu;
use crate::AppState;

use super::MenuState;

pub struct MenuSystemsPlugin;
impl Plugin for MenuSystemsPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((MainMenuPlugin, OptionsMenuPlugin, KeyboardMenuPlugin))
			.add_systems(OnEnter(AppState::Game), exit_menu)
			.add_systems(OnEnter(MenuState::None), despawn_menu);
	}
}

pub fn exit_menu(mut menu_state: ResMut<NextState<MenuState>>) {
	menu_state.set(MenuState::None);
}

pub fn despawn_menu(
	mut commands: Commands,
	q_main_menu: Query<Entity, With<MainMenu>>,
	q_camera: Query<Entity, With<MainMenuCamera>>,
) {
	if let Ok(main_menu_entity) = q_main_menu.get_single() {
		commands.entity(main_menu_entity).despawn_recursive();
	}

	if let Ok(camera_entity) = q_camera.get_single() {
		commands.entity(camera_entity).despawn_recursive();
	}
}
