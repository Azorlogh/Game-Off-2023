use bevy::prelude::*;

pub struct NutritionPlugin;
impl Plugin for NutritionPlugin {
	fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Hydration(pub f32);

#[derive(Component)]
pub struct Glucose(pub f32);
