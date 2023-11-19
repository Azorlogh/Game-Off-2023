use bevy::prelude::*;

pub(crate) mod enemies;
pub(crate) mod food;
pub(crate) mod hud;
pub(crate) mod movement;
pub(crate) mod player;

use enemies::EnemyPlugin;
use food::FoodPlugin;
use hud::HudPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			MovementPlugin,
			PlayerPlugin,
			FoodPlugin,
			EnemyPlugin,
			HudPlugin,
		));
	}
}
