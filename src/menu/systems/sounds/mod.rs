use bevy::prelude::*;

pub mod interactions;
pub mod layout;

use interactions::*;
use layout::*;

use crate::menu::MenuState;

// SOunds Menu
#[derive(Component)]
pub struct SoundsMenu;

#[derive(Component)]
pub struct SoundsdBack;

pub struct SoundsMenuPlugin;
impl Plugin for SoundsMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (interact_back_button,))
			.add_systems(OnEnter(MenuState::Sounds), spawn_menu)
			.add_systems(OnExit(MenuState::Sounds), despawn_menu);
	}
}
