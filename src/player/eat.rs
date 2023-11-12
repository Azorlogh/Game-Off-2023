use bevy::{ecs::query::QueryParIter, prelude::*};
use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext};

use crate::food::{self, FoodStats};
use crate::{food::Food, input::Inputs};

use super::nutrition::{Glucose, Hydration};
use super::MainCamera;
use super::Player;

pub fn player_eat(
	rapier_context: Res<RapierContext>,
	inputs: Res<Inputs>,
	q_food: Query<&FoodStats, With<Food>>,
	q_camera_player: Query<&GlobalTransform, With<MainCamera>>,
	mut q_player: Query<(Entity, (&mut Glucose, &mut Hydration)), With<Player>>,
	mut commands: Commands,
	mut gizmos: Gizmos,
) {
	if !inputs.eat {
		return;
	}

	let Ok(global_transform) = q_camera_player.get_single() else {
		return;
	};

	let Ok((player_entity, (mut glucose, mut hydration))) = q_player.get_single_mut() else {
		return;
	};

	let ray_pos = global_transform.translation() - Vec3::Y * 0.1;
	let ray_dir = global_transform.forward();
	let max_toi = 4.0;
	let solid = true;
	let filter: QueryFilter = QueryFilter::new().exclude_rigid_body(player_entity);

	gizmos.ray(ray_pos, ray_dir * max_toi, Color::GREEN);

	if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
		// The first collider hit has the entity `entity` and it hit after
		// the ray travelled a distance equal to `ray_dir * toi`.
		let hit_point = ray_pos + ray_dir * toi;

		println!("Entity {:?} hit at point {}", entity, hit_point);

		if let Ok(food_stats) = q_food.get(entity) {
			// player
			glucose.0 += food_stats.glucose;
			hydration.0 += food_stats.hydration;
			commands.entity(entity).despawn_recursive();
		}
	}
}
