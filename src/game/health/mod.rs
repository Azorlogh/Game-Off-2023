use bevy::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use super::scaling::Scaling;
use crate::game::player::camera::PlayerCamera;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Health>()
			.add_event::<Hit>()
			.add_systems(Update, (display_health, take_hit, die));
	}
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Health {
	pub current: f32,
	pub max: f32,
}

impl Health {
	pub fn heal(&mut self, amt: f32) {
		self.current = (self.current + amt).clamp(0.0, self.max);
	}
}

#[derive(Event)]
pub struct Hit {
	pub target: Entity,
	pub damage: f32,
}

fn take_hit(mut ev_take_hit: EventReader<Hit>, mut q_health: Query<(&mut Health, &Scaling)>) {
	for ev in ev_take_hit.iter() {
		if let Ok((mut health, scaling)) = q_health.get_mut(ev.target) {
			health.current = health.current - ev.damage / scaling.0;
		}
	}
}

#[derive(Component)]
pub struct Dead;

fn die(mut cmds: Commands, q_agent: Query<(Entity, &Health), Without<Dead>>) {
	for (entity, health) in &q_agent {
		if health.current <= 0.0 {
			cmds.entity(entity).insert(Dead);
		}
	}
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
		let size = 1.0;
		let healthbar_length = HEALTHBAR_LENGTH * size;
		let healthbar_height = 0.1 * size;
		painter.thickness = 0.02 * size;
		painter.color = Color::GRAY;
		let healthbar_pos = transform.translation() + Vec3::Y * healthbar_height;
		let healthbar_left = healthbar_pos - camera_tr.right() * healthbar_length / 2.0;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * healthbar_length,
		);

		let health_ratio = (health.current / health.max).max(0.0);

		painter.color = Color::RED;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * (healthbar_length * health_ratio),
		);
	}
}
