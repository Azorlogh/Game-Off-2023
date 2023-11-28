use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Food;

#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct FoodStats {
	pub calories: f32,
	pub health: f32,
}
impl Default for FoodStats {
	fn default() -> Self {
		Self {
			calories: 0.0,
			health: 0.0,
		}
	}
}

#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct FoodProperties {
	pub bites: u32,
	pub total_bites: u32,
	pub time_per_bite: f32,
}

#[derive(Component, Reflect)]
pub struct FoodSize(pub f32);
