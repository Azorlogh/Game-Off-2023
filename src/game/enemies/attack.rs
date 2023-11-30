use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
use serde::Deserialize;

use super::{roaming::RoamingState, Enemy, EnemyState};
use crate::{
	game::{health::Hit, movement::MovementInput, player::Player, scaling::Scaling},
	DEBUG,
};

#[derive(Component)]
pub struct SpottingRange(pub f32);

#[derive(Debug, Clone, Component, Deserialize)]
pub struct AttackStats {
	pub range: f32,
	pub speed: f32,
	pub damage: f32,
}

#[derive(Clone, Copy, Reflect)]
pub enum AttackState {
	Chasing,
	Attacking(f32),
}

/// Triggers attack mode when the player is near
/// The spotting range depends on both the enemy's and the player's scales
/// Therefore, a large enemy won't see a tiny player unless it's really close
pub fn enemy_start_chase(
	q_player: Query<(Entity, &GlobalTransform, &Scaling), With<Player>>,
	mut q_enemies: Query<
		(&mut EnemyState, &GlobalTransform, &SpottingRange, &Scaling),
		With<Enemy>,
	>,
) {
	for (mut enemy_state, enemy_tr, spotting_range, scaling) in q_enemies
		.iter_mut()
		.filter(|(state, _, _, _)| matches!(**state, EnemyState::Roaming(_)))
	{
		for (player_entity, player_tr, player_scaling) in &q_player {
			if enemy_tr.translation().distance(player_tr.translation())
				< spotting_range.0 * scaling.0 * player_scaling.0
			{
				*enemy_state = EnemyState::Attacking(player_entity, AttackState::Chasing);
			}
		}
	}
}

pub fn enemy_chase(
	q_global_transform: Query<&GlobalTransform>,
	q_scaling: Query<&Scaling>,
	mut q_enemies: Query<(
		&mut EnemyState,
		Entity,
		&mut MovementInput,
		&SpottingRange,
		&Scaling,
	)>,
) {
	for (mut state, enemy_entity, mut input, spotting_range, scaling) in &mut q_enemies {
		let EnemyState::Attacking(target, attack_state) = *state else {
			continue;
		};

		let enemy_gtr = q_global_transform.get(enemy_entity).unwrap();
		let target_gtr = q_global_transform.get(target).unwrap();
		let to_target = target_gtr.translation() - enemy_gtr.translation();

		let target_scaling = q_scaling.get(target).unwrap();

		if to_target.length() > spotting_range.0 * scaling.0 * target_scaling.0 * 3.0 {
			*state = EnemyState::Roaming(RoamingState::Waiting { remaining: 4.0 });
		}

		let to_target_dir = to_target.normalize() * Vec3::new(1.0, 0.0, 1.0);

		if let AttackState::Chasing = attack_state {
			input.0 = to_target_dir.xz();
		} else {
			input.0 = to_target_dir.xz() * 0.0000001; // hack: just so the auto-align system works on this
		}
	}
}

pub fn enemy_attack(
	time: Res<Time>,
	mut q_enemies: Query<(&mut EnemyState, &Transform, &AttackStats, &Scaling)>,
	rapier_context: Res<RapierContext>,
	mut gizmos: Gizmos,
	mut ev_hit: EventWriter<Hit>,
) {
	for (mut state, enemy_tr, stats, scaling) in &mut q_enemies {
		let EnemyState::Attacking(target, ref mut attack_state) = *state else {
			continue;
		};

		let enemy_pos = enemy_tr.translation;

		if DEBUG {
			gizmos.sphere(enemy_pos, Quat::default(), stats.range, Color::RED);
		}

		let player_within_range = |range: f32| {
			rapier_context
				.intersection_with_shape(
					enemy_pos,
					Quat::default(),
					&Collider::ball(range),
					QueryFilter::new().predicate(&|e| e == target),
				)
				.is_some()
		};

		let can_start_attack = player_within_range(stats.range * scaling.0 * 0.5); // We wait until we have some margin to start attacking
		let can_still_attack = player_within_range(stats.range * scaling.0);

		match attack_state {
			AttackState::Chasing if can_start_attack => {
				*attack_state = AttackState::Attacking(0.0);
			}
			AttackState::Attacking(attack_time) if *attack_time > stats.speed => {
				*attack_state = AttackState::Chasing;
				if can_still_attack {
					ev_hit.send(Hit {
						target,
						damage: stats.damage,
					});
				}
			}
			AttackState::Attacking(attack_time) => {
				*attack_time += time.delta_seconds();
			}
			_ => {}
		}
	}
}
