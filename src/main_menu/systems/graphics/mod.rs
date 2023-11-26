use bevy::prelude::*;

pub mod interactions;
pub mod layout;

use interactions::*;
use layout::*;

use crate::main_menu::MenuState;

// Graphics Menu
#[derive(Component)]
pub struct GraphicsMenu;

#[derive(Component)]
pub struct GraphicsBack;

pub struct GraphicsMenuPlugin;
impl Plugin for GraphicsMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (interact_back_button,))
			.add_systems(OnEnter(MenuState::Graphics), spawn_menu)
			.add_systems(OnExit(MenuState::Graphics), despawn_menu);
	}
}
