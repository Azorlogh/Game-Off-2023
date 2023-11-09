use bevy::prelude::*;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
	fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Health {
	pub current: f32,
	pub max: f32,
}
