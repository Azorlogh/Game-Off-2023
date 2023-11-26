use bevy::prelude::*;

pub struct ScalingPlugin;
impl Plugin for ScalingPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Scaling>()
			.add_systems(Update, scale_agents);
	}
}

#[derive(Component, Reflect)]
pub struct Scaling(pub f32);

impl Default for Scaling {
	fn default() -> Self {
		Self(1.0)
	}
}

fn scale_agents(mut q_player: Query<(&mut Transform, &Scaling)>) {
	for (mut transform, scaling) in &mut q_player {
		transform.scale = Vec3::splat(scaling.0);
	}
}
