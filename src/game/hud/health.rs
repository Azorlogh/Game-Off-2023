use bevy::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use crate::game::player::camera::PlayerCamera;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<Hit>()
			.add_systems(Update, (display_health, take_hit));
	}
}

#[derive(Component)]
pub struct Health {
	pub current: u32,
	pub max: u32,
}

#[derive(Component)]
pub struct HideHealthBar;

fn display_health(
	mut painter: ShapePainter,
	query: Query<(&Health, &GlobalTransform), Without<HideHealthBar>>,
	q_camera: Query<&GlobalTransform, With<PlayerCamera>>,
) {
	const HEALTHBAR_LENGTH: f32 = 0.25;
	let Ok(camera_tr) = q_camera.get_single() else {
		return;
	};

	for (health, transform) in &query {
		painter.thickness = 0.02;
		painter.color = Color::GRAY;
		let healthbar_pos = transform.translation() + Vec3::Y * 0.5;
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

#[derive(Event)]
pub struct Hit {
	pub target: Entity,
	pub damage: u32,
}

fn take_hit(mut ev_take_hit: EventReader<Hit>, mut q_health: Query<&mut Health>) {
	for ev in ev_take_hit.iter() {
		if let Ok(mut health) = q_health.get_mut(ev.target) {
			health.current = health.current.saturating_sub(ev.damage);
		}
	}
}
