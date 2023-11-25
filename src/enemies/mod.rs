//!
//! TODO: Let 'em jump?
//!

mod model;
pub mod template;

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::{
	dynamics::{LockedAxes, Velocity},
	pipeline::QueryFilter,
	plugin::RapierContext,
	prelude::RigidBody,
};
use serde::Deserialize;

use self::{
	model::EnemyModelPlugin,
	template::{EnemyAssetLoader, EnemyTemplate},
};
use crate::{
	health::{Health, Hit},
	movement::{MovementInput, OnGround, Speed},
	player::Player,
	scaling::Scaling,
	GameAssets, GameState,
};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnemyModelPlugin)
			.register_type::<EnemyState>()
			.add_asset::<EnemyTemplate>()
			.add_asset_loader(EnemyAssetLoader)
			.add_event::<SpawnEnemy>()
			.add_systems(OnExit(GameState::Loading), setup)
			.add_systems(
				Update,
				(enemy_spawn, enemy_start_chase, enemy_chase, enemy_attack),
			);
	}
}

#[derive(Event)]
pub struct SpawnEnemy {
	template: Handle<EnemyTemplate>,
	pos: Vec3,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct SpottingRange(f32);

#[derive(Debug, Clone, Component, Deserialize)]
pub struct AttackStats {
	range: f32,
	speed: f32,
	damage: u32,
}

#[derive(Component, Reflect)]
pub enum EnemyState {
	Idle,
	Attacking(Entity, AttackState),
}

#[derive(Clone, Copy, Reflect)]
pub enum AttackState {
	Chasing,
	Attacking(f32),
}

fn setup(mut ev_spawn_enemy: EventWriter<SpawnEnemy>, assets: Res<GameAssets>) {
	// Summon the rat army
	// const N: usize = 16;
	// for i in 0..N {
	// 	for j in 0..N {
	// 		for k in 0..4 {
	// 			ev_spawn_enemy.send(SpawnEnemy {
	// 				pos: Vec3::new(i as f32, k as f32 + 20.0, j as f32),
	// 				template: assets.enemies["enemies/rat.enemy.ron"].clone_weak(),
	// 			});
	// 		}
	// 	}
	// }

	// Summon random enemies
	// const N: usize = 32;
	// const RANGE: f32 = 15.0;
	// for _ in 0..N {
	// 	let template = match rand::random::<u8>() % 3 {
	// 		0 => "spider",
	// 		1 => "rat",
	// 		2 => "snake",
	// 		_ => unreachable!(),
	// 	};
	// 	let x = (rand::random::<f32>() - 0.5) * RANGE;
	// 	let z = (rand::random::<f32>() - 0.5) * RANGE;
	// 	ev_spawn_enemy.send(SpawnEnemy {
	// 		pos: Vec3::new(x, rand::random::<f32>() * 100.0, z),
	// 		template: assets.enemies[&format!("enemies/{template}.enemy.ron")].clone_weak(),
	// 	});
	// }

	// Summon one enemy
	ev_spawn_enemy.send(SpawnEnemy {
		pos: Vec3::new(0.0, 0.0, -5.0),
		template: assets.enemies["enemies/spider.enemy.ron"].clone_weak(),
	});
}

fn enemy_spawn(
	mut cmds: Commands,
	mut ev_spawn_enemy: EventReader<SpawnEnemy>,
	enemy_assets: Res<Assets<EnemyTemplate>>,
) {
	for ev in ev_spawn_enemy.iter() {
		let template = enemy_assets.get(&ev.template).unwrap();
		cmds.spawn((
			Name::new("Enemy"),
			Enemy,
			ev.template.clone_weak(),
			SpatialBundle::from_transform(Transform::from_translation(ev.pos)),
			RigidBody::Dynamic,
			EnemyState::Idle,
			Velocity::default(),
			LockedAxes::ROTATION_LOCKED,
			Scaling(1.0),
			(
				OnGround(true),
				MovementInput::default(),
				Speed(template.speed),
			),
			template.attack_stats.clone(),
			SpottingRange(template.spotting_range),
			Health {
				current: template.health,
				max: template.health,
			},
		))
		.with_children(|cmds| {
			cmds.spawn((TransformBundle::default(), template.collider.clone()));
		});
	}
}

fn enemy_start_chase(
	q_player: Query<(Entity, &GlobalTransform), With<Player>>,
	mut q_enemies: Query<(&mut EnemyState, &GlobalTransform, &SpottingRange), With<Enemy>>,
) {
	for (mut enemy_state, enemy_tr, spotting_range) in q_enemies
		.iter_mut()
		.filter(|(state, _, _)| matches!(**state, EnemyState::Idle))
	{
		for (player_entity, player_tr) in &q_player {
			if enemy_tr.translation().distance(player_tr.translation()) < spotting_range.0 {
				*enemy_state = EnemyState::Attacking(player_entity, AttackState::Chasing);
			}
		}
	}
}

fn enemy_chase(
	q_global_transform: Query<&GlobalTransform>,
	mut q_enemies: Query<(&EnemyState, Entity, &mut Transform, &mut MovementInput)>,
) {
	for (state, enemy_entity, mut enemy_tr, mut input) in &mut q_enemies {
		let EnemyState::Attacking(target, attack_state) = *state else {
			continue;
		};

		let enemy_gtr = q_global_transform.get(enemy_entity).unwrap();
		let target_gtr = q_global_transform.get(target).unwrap();
		let to_target_dir = (target_gtr.translation() - enemy_gtr.translation()).normalize()
			* Vec3::new(1.0, 0.0, 1.0);

		let axis = enemy_gtr.forward().cross(to_target_dir);
		enemy_tr.rotate(Quat::from_scaled_axis(axis * 0.1));

		if let AttackState::Chasing = attack_state {
			input.0 = to_target_dir.xz();
		} else {
			input.0 = default();
		}
	}
}

fn enemy_attack(
	time: Res<Time>,
	q_global_transform: Query<&GlobalTransform>,
	mut q_enemies: Query<(&mut EnemyState, &Transform, &AttackStats)>,
	mut ev_hit: EventWriter<Hit>,
	rapier_context: Res<RapierContext>,
	mut gizmos: Gizmos,
) {
	for (mut state, enemy_tr, stats) in &mut q_enemies {
		let EnemyState::Attacking(target, ref mut attack_state) = *state else {
			continue;
		};

		let target_pos = q_global_transform.get(target).unwrap().translation();
		let enemy_pos = enemy_tr.translation;
		// let target_distance = enemy_pos.distance(target_pos);

		gizmos.ray(
			enemy_pos,
			((target_pos - enemy_pos) * Vec3::new(1.0, 0.0, 1.0)).normalize() * stats.range,
			Color::RED,
		);

		let mut can_attack = false;
		rapier_context.intersections_with_ray(
			enemy_pos,
			((target_pos - enemy_pos) * Vec3::new(1.0, 0.0, 1.0)).normalize(),
			stats.range,
			true,
			QueryFilter::new().predicate(&|e| {
				println!("{e:?}");
				e == target
			}),
			|_, _| {
				can_attack = true;
				false
			},
		);
		println!("can attack {can_attack}, {target:?}");

		match attack_state {
			AttackState::Chasing if can_attack => {
				*attack_state = AttackState::Attacking(0.0);
			}
			AttackState::Attacking(attack_time) if *attack_time > stats.speed => {
				*attack_state = AttackState::Chasing;
				if can_attack {
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
