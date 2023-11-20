use bevy::prelude::*;

pub struct ScalingPlugin;
impl Plugin for ScalingPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Scaling>();
	}
}

#[derive(Component, Reflect)]
pub struct Scaling(pub f32);

impl Default for Scaling {
	fn default() -> Self {
		Self(1.0)
	}
}
