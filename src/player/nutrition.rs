use bevy::prelude::*;

pub struct NutritionPlugin;
impl Plugin for NutritionPlugin {
	fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Hydration(pub i32);

#[derive(Component)]
pub struct Glucose(pub i32);
