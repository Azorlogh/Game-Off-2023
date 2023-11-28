use bevy::{prelude::*, render::primitives::Aabb};
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use super::scaling::Scaling;
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
	query: Query<(&Health, &GlobalTransform, &Aabb), Without<HideHealthBar>>,
	q_camera: Query<&GlobalTransform, With<PlayerCamera>>,
) {
	const HEALTHBAR_LENGTH: f32 = 0.25;
	let Ok(camera_tr) = q_camera.get_single() else {
		return;
	};

	for (health, transform, aabb) in &query {
		let size = aabb.half_extents.max_element();
		let healthbar_length = HEALTHBAR_LENGTH * size;
		let healthbar_height = 0.5 * size;
		painter.thickness = 0.02 * size;
		painter.color = Color::GRAY;
		let healthbar_pos = transform.translation() + Vec3::Y * healthbar_height;
		let healthbar_left = healthbar_pos - camera_tr.right() * healthbar_length / 2.0;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * healthbar_length,
		);

		let health_ratio = health.current as f32 / health.max as f32;

		painter.color = Color::RED;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * (healthbar_length * health_ratio),
		);
	}
}

#[derive(Event)]
pub struct Hit {
	pub target: Entity,
	pub damage: u32,
}

fn take_hit(mut ev_take_hit: EventReader<Hit>, mut q_health: Query<(&mut Health, &Scaling)>) {
	for ev in ev_take_hit.iter() {
		if let Ok((mut health, scaling)) = q_health.get_mut(ev.target) {
			health.current = health
				.current
				.saturating_sub((ev.damage as f32 / scaling.0) as u32);
		}
	}
}
