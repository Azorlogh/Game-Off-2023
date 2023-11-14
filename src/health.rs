use bevy::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use crate::player::MainCamera;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, display_health);
	}
}

#[derive(Component)]
pub struct Health {
	pub current: u32,
	pub max: u32,
}

fn display_health(
	mut painter: ShapePainter,
	query: Query<(&Health, &GlobalTransform)>,
	q_camera: Query<&GlobalTransform, With<MainCamera>>,
) {
	const HEALTHBAR_LENGTH: f32 = 0.25;
	let Ok(camera_tr) = q_camera.get_single() else {
		return;
	};

	for (health, transform) in &query {
		painter.thickness = 0.02;
		painter.color = Color::GRAY;
		let healthbar_pos = transform.translation() + Vec3::Y * 0.3;
		let healthbar_left = healthbar_pos - camera_tr.right() * HEALTHBAR_LENGTH / 2.0;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * HEALTHBAR_LENGTH,
		);

		let health_ratio = health.current as f32 / health.max as f32;

		painter.color = Color::RED;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * (HEALTHBAR_LENGTH * health_ratio),
		);
	}
}
