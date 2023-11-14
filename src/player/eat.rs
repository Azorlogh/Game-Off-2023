use bevy::prelude::*;
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

use crate::food::{FoodProperties, FoodStats};
use crate::{food::Food, input::Inputs};

use super::nutrition::{Glucose, Hydration};
use super::MainCamera;
use super::Player;

#[derive(Default, Debug)]
pub enum AttractingState {
	Attracting,
	#[default]
	Idle,
}
pub fn player_attract_and_eat(
	rappier_contect: Res<RapierContext>,
	inputs: Res<Inputs>,
	mut q_food: Query<(&FoodStats, &mut FoodProperties, &mut Transform), With<Food>>,
	mut q_player: Query<(Entity, &Transform), With<Player>>,
	mut commands: Commands,
	mut gizmos: Gizmos,
	mut attracting_state: Local<AttractingState>,
	time: Res<Time>,
) {
	let Ok((player_entity, player_transform)) = q_player.get_single_mut() else {
		return;
	};

	if inputs.attract {
		*attracting_state = AttractingState::Attracting;
		for (_, _, mut food_transform) in q_food.iter_mut() {
			let distance = player_transform.translation.distance(food_transform.translation);
			if distance < 0.001 {
				// move food to player
				let direction = (player_transform.translation - food_transform.translation).normalize();
				let speed = 0.002 * time.delta_seconds();
				let new_position = food_transform.translation + direction * speed;
				food_transform.translation = new_position;
			}
		}

	} else { *attracting_state = AttractingState::Idle; }

	//eat food in crcle in range of player :
	for (food_stats, mut food_properties, mut food_transform) in q_food.iter_mut() {
	
	}

}


// pub fn player_eat(
// 	rapier_context: Res<RapierContext>,
// 	inputs: Res<Inputs>,
// 	mut q_food: Query<(&FoodStats, &mut FoodProperties), With<Food>>,
// 	q_camera_player: Query<&GlobalTransform, With<MainCamera>>,
// 	mut q_player: Query<(Entity, (&mut Glucose, &mut Hydration)), With<Player>>,
// 	mut commands: Commands,
// 	mut gizmos: Gizmos,
// 	mut eating_state: Local<EatingState>,
// 	time: Res<Time>,
// ) {
// 	if !inputs.eat {
// 		*eating_state = EatingState::Idle;
// 		return;
// 	}

// 	let Ok(global_transform) = q_camera_player.get_single() else {
// 		return;
// 	};

// 	let Ok((player_entity, (mut glucose, mut hydration))) = q_player.get_single_mut() else {
// 		return;
// 	};

// 	let ray_pos = global_transform.translation() - Vec3::Y * 0.1;
// 	let ray_dir = global_transform.forward();
// 	let max_toi = 4.0;
// 	let solid = true;
// 	let filter: QueryFilter = QueryFilter::new().exclude_rigid_body(player_entity);

// 	gizmos.ray(ray_pos, ray_dir * max_toi, Color::GREEN);

// 	if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
// 		// The first collider hit has the entity `entity` and it hit after
// 		// the ray travelled a distance equal to `ray_dir * toi`.
// 		let hit_point = ray_pos + ray_dir * toi;

// 		debug!("Entity {:?} hit at point {}", entity, hit_point);

// 		debug!("Eating state: {:?}", eating_state);

// 		if let Ok((food_stats, mut food_properties)) = q_food.get_mut(entity) {
// 			match &*eating_state {
// 				EatingState::Eating(eating_entity, eating_since) => {
// 					if *eating_entity != entity {
// 						// stop last eating and start new eating
// 						*eating_state = EatingState::Eating(entity, 0.0);
// 					} else {
// 						// continue eating
// 						let new_time = eating_since + time.delta_seconds();
// 						if new_time > food_properties.time_per_bite {
// 							*eating_state = EatingState::Eating(entity, 0.0);
// 							glucose.0 += food_stats.glucose;
// 							hydration.0 += food_stats.hydration;
// 							food_properties.health -= 1;
// 							if food_properties.health == 0 {
// 								*eating_state = EatingState::Idle;
// 								commands.entity(entity).despawn_recursive();
// 							}
// 						} else {
// 							*eating_state = EatingState::Eating(entity, new_time);
// 						}
// 					}
// 				}
// 				EatingState::Idle => {
// 					*eating_state = EatingState::Eating(entity, 0.0);
// 				}
// 			};
// 		} else {
// 			*eating_state = EatingState::Idle;
// 		}
// 	} else {
// 		*eating_state = EatingState::Idle;
// 	}
// }
