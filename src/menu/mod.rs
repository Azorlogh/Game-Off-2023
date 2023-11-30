pub(crate) mod components;
pub(crate) mod styles;
mod systems;

use bevy::prelude::*;
use systems::main::layout::*;

use self::systems::MenuSystemsPlugin;
use crate::AppState;

#[derive(Component)]
pub struct ColoredButton;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MenuState {
	#[default]
	Main,
	Options,
	Graphics,
	Sounds,
	Keyboard,
	None,
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<MenuState>()
			.add_plugins(MenuSystemsPlugin)
			.add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
			.add_systems(Update, styles::highlight_button_interactions);
	}
}
