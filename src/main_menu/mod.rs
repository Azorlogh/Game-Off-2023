pub(crate) mod components;
mod styles;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::main::layout::*;

use self::systems::MenuSystemsPlugin;

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
			.add_systems(OnEnter(AppState::MainMenu), enter_main_menu);
	}
}
