use bevy::{prelude::*, render::primitives::Aabb};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::LinePainter};

use super::{
	components::{Food, FoodProperties, FoodSize},
	SpawnFood,
};
use crate::game::player::camera::PlayerCamera;

pub fn setup_food(mut _ev_spawn_food: EventWriter<SpawnFood>) {
	// ev_spawn_food.send(SpawnFood {
	// 	name: String::from("Apple"),
	// 	model: String::from("models/foods/glb/Apple.glb#Scene0"),
	// 	stats: FoodStats {
	// 		calories: 5.0,
	// 		health: 4.0,
	// 	},
	// 	properties: FoodProperties {
	// 		bites: 3,
	// 		total_bites: 3,
	// 		time_per_bite: 1.0,
	// 	},
	// 	position: Vec3::new(0.0, 10.0, 0.0),
	// 	scale_factor: 0.1,
	// })
}

pub fn calculate_food_size(
	mut cmds: Commands,
	q_foods: Query<Entity, (Without<FoodSize>, With<Food>)>,
	q_children: Query<&Children>,
	mut q_aabb: Query<&Aabb>,
) {
	for entity in &q_foods {
		for descendant_e in q_children.iter_descendants(entity) {
			if let Ok(aabb) = q_aabb.get_mut(descendant_e) {
				cmds.entity(entity)
					.insert(FoodSize(aabb.half_extents.max_element()));
			}
		}
	}
}

pub fn display_health_food(
	mut painter: ShapePainter,
	query: Query<(&FoodProperties, &GlobalTransform, &FoodSize), With<Food>>,
	q_camera: Query<&GlobalTransform, With<PlayerCamera>>,
) {
	const HEALTHBAR_LENGTH: f32 = 2.0;
	let Ok(camera_tr) = q_camera.get_single() else {
		return;
	};

	for (food_properties, transform, size) in &query {
		let healthbar_length = HEALTHBAR_LENGTH * size.0;
		let healthbar_height = 1.5 * size.0;
		painter.thickness = 0.2 * size.0;
		painter.color = Color::GRAY;
		let healthbar_pos = transform.translation() + Vec3::Y * healthbar_height;
		let healthbar_left = healthbar_pos - camera_tr.right() * healthbar_length / 2.0;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * healthbar_length,
		);

		let health_ratio = food_properties.bites as f32 / food_properties.total_bites as f32;

		painter.color = Color::ORANGE;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * (healthbar_length * health_ratio),
		);
	}
}
