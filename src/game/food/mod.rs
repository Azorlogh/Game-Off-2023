use bevy::prelude::*;
use systems::{display_health_food, setup_food};
pub(crate) mod systems;

use components::*;
pub(crate) mod components;

pub struct FoodPlugin;

// TODO: refactor, on utilisera que blender
impl Plugin for FoodPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_food)
			.add_systems(Update, (display_health_food,))
			.add_event::<SpawnFood>()
			.register_type::<FoodStats>()
			.register_type::<FoodProperties>()
			.register_type::<Food>();
	}
}

#[derive(Event)]
pub struct SpawnFood {
	pub name: String,
	pub model: String,
	pub stats: FoodStats,
	pub position: Vec3,
	pub scale_factor: f32,
	pub properties: FoodProperties,
}
