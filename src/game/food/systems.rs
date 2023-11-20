use bevy::prelude::*;

use bevy_vector_shapes::{prelude::ShapePainter, shapes::LinePainter};

use crate::game::player::components::PlayerCamera;

use super::components::{Food, FoodProperties, FoodStats, SpawnFood};

pub fn setup_food(mut ev_spawn_food: EventWriter<SpawnFood>) {
	ev_spawn_food.send(SpawnFood {
		name: String::from("Apple"),
		model: String::from("models/foods/glb/Apple.glb#Scene0"),
		stats: FoodStats {
			hydration: 3,
			glucose: 7,
			fat: 2,
			health: 4,
		},
		properties: FoodProperties {
			health: 3,
			total_health: 3,
			time_per_bite: 1.0,
		},
		position: Vec3::new(0.0, 10.0, 0.0),
		scale_factor: 0.1,
	})
}

pub fn display_health_food(
	mut painter: ShapePainter,
	query: Query<(&FoodProperties, &GlobalTransform), With<Food>>,
	q_camera: Query<&GlobalTransform, With<PlayerCamera>>,
) {
	const HEALTHBAR_LENGTH: f32 = 0.25;
	let Ok(camera_tr) = q_camera.get_single() else {
		return;
	};

	for (food_properties, transform) in &query {
		painter.thickness = 0.02;
		painter.color = Color::GRAY;
		let healthbar_pos = transform.translation() + Vec3::Y * 0.1;
		let healthbar_left = healthbar_pos - camera_tr.right() * HEALTHBAR_LENGTH / 2.0;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * HEALTHBAR_LENGTH,
		);

		let health_ratio = food_properties.health as f32 / food_properties.total_health as f32;

		painter.color = Color::ORANGE;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * (HEALTHBAR_LENGTH * health_ratio),
		);
	}
}
