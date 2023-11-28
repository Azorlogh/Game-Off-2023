use bevy::prelude::*;
use bevy_rapier3d::{
	dynamics::{LockedAxes, Velocity},
	prelude::RigidBody,
};

use super::{
	super::DespawnOnExitGame, attack::SpottingRange, template::EnemyTemplate, Enemy, EnemyState,
};
use crate::{
	game::{
		health::Health,
		movement::{MovementInput, OnGround, Speed},
		scaling::Scaling,
	},
	GameAssets,
};

#[derive(Event)]
pub struct SpawnEnemy {
	pub template: Handle<EnemyTemplate>,
	pub pos: Vec3,
}

pub fn setup(mut ev_spawn_enemy: EventWriter<SpawnEnemy>, assets: Res<GameAssets>) {
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
		pos: Vec3::new(0.0, 0.5, 4.0),
		template: assets.enemies["enemies/spider.enemy.ron"].clone_weak(),
	});
	ev_spawn_enemy.send(SpawnEnemy {
		pos: Vec3::new(-5.0, 0.5, -2.0),
		template: assets.enemies["enemies/rat.enemy.ron"].clone_weak(),
	});
	ev_spawn_enemy.send(SpawnEnemy {
		pos: Vec3::new(4.0, 0.5, -2.0),
		template: assets.enemies["enemies/snake.enemy.ron"].clone_weak(),
	});
}

pub fn enemy_spawn(
	mut cmds: Commands,
	mut ev_spawn_enemy: EventReader<SpawnEnemy>,
	enemy_assets: Res<Assets<EnemyTemplate>>,
) {
	for ev in ev_spawn_enemy.iter() {
		let template = enemy_assets.get(&ev.template).unwrap();
		cmds.spawn((
			Name::new("Enemy"),
			Enemy,
			DespawnOnExitGame,
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
