use bevy::prelude::*;
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

use super::{calories::Calories, camera::PlayerCamera, Player};
use crate::{
	game::{
		food::components::{Food, FoodProperties, FoodStats},
		health::Health,
		scaling::Scaling,
	},
	input::Inputs,
};

const EATING_RANGE: f32 = 3.0;
const RAY_SOLID: bool = true;

#[derive(Default, Debug, Resource)]
pub enum EatingState {
	Eating(Entity, f32),
	#[default]
	Idle,
}

pub fn player_eat(
	mut cmds: Commands,
	asset_server: Res<AssetServer>,
	rapier_context: Res<RapierContext>,
	inputs: Res<Inputs>,
	mut q_food: Query<(&FoodStats, &mut FoodProperties), With<Food>>,
	q_camera_player: Query<&GlobalTransform, With<PlayerCamera>>,
	mut q_player: Query<(Entity, &mut Calories, &mut Health, &Scaling), With<Player>>,
	mut commands: Commands,
	mut gizmos: Gizmos,
	mut eating_state: ResMut<EatingState>,
	time: Res<Time>,
) {
	if !inputs.eat {
		*eating_state = EatingState::Idle;
		return;
	}

	let Ok(player_transform) = q_camera_player.get_single() else {
		return;
	};

	let Ok((player_entity, mut calories, mut health, scaling)) = q_player.get_single_mut() else {
		return;
	};

	let ray_pos = player_transform.translation();
	let ray_dir = player_transform.forward();
	let max_toi = EATING_RANGE * scaling.0;
	let solid = RAY_SOLID;
	let filter: QueryFilter = QueryFilter::new().exclude_rigid_body(player_entity);

	gizmos.ray(ray_pos, ray_dir * max_toi, Color::GREEN);

	if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
		// The first collider hit has the entity `entity` and it hit after
		// the ray travelled a distance equal to `ray_dir * toi`.
		let hit_point = ray_pos + ray_dir * toi;

		debug!("Entity {:?} hit at point {}", entity, hit_point);

		debug!("Eating state: {:?}", eating_state);

		if let Ok((food_stats, mut food_properties)) = q_food.get_mut(entity) {
			match &*eating_state {
				EatingState::Eating(eating_entity, eating_since) => {
					if *eating_entity != entity {
						// stop last eating and start new eating
						*eating_state = EatingState::Eating(entity, 0.0);
					} else {
						// continue eating
						let new_time = eating_since + time.delta_seconds();
						if new_time > food_properties.time_per_bite {
							cmds.spawn(AudioBundle {
								source: asset_server.load("sounds/bite.ogg"),
								settings: PlaybackSettings {
									speed: rand::random::<f32>() * 0.2 + 0.9,
									..default()
								},
							});

							*eating_state = EatingState::Eating(entity, 0.0);
							calories.0 += food_stats.calories / food_properties.total_bites as f32;
							health.heal(food_stats.calories * 0.1 / scaling.0);
							food_properties.bites -= 1;
							if food_properties.bites == 0 {
								*eating_state = EatingState::Idle;
								commands.entity(entity).despawn_recursive();
							}
						} else {
							*eating_state = EatingState::Eating(entity, new_time);
						}
					}
				}
				EatingState::Idle => {
					*eating_state = EatingState::Eating(entity, 0.0);
				}
			};
		} else {
			*eating_state = EatingState::Idle;
		}
	} else {
		*eating_state = EatingState::Idle;
	}
}
