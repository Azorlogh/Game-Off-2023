use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Food;

#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct FoodStats {
	pub hydration: i32,
	pub glucose: i32,
	pub fat: i32,
	pub health: i32,
}
impl Default for FoodStats {
	fn default() -> Self {
		Self {
			hydration: 1,
			glucose: 1,
			fat: 1,
			health: 0,
		}
	}
}

#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct FoodProperties {
	pub health: u32,
	pub total_health: u32,
	pub time_per_bite: f32,
}
