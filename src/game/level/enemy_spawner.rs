use bevy::{prelude::*, render::primitives::Aabb};

use crate::game::{enemies::spawn::SpawnEnemy, GameAssets};

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct EnemySpawner {
	name: String,
	density: f32,
}

pub fn spawn_enemies(
	mut cmds: Commands,
	q_added_blueprints_spawners: Query<(Entity, &EnemySpawner)>,
	q_children: Query<&Children>,
	mut q_aabb: Query<&Aabb>,
	mut ev_spawn_enemy: EventWriter<SpawnEnemy>,
	assets: Res<GameAssets>,
) {
	for (entity, spawner) in &q_added_blueprints_spawners {
		for descendant_e in q_children.iter_descendants(entity) {
			if let Ok(aabb) = q_aabb.get_mut(descendant_e) {
				let sides = aabb.half_extents * 2.0;
				let nb_instances = (sides.x * sides.z * spawner.density) as usize;
				println!("Spawning {} instances!", nb_instances);
				for _ in 0..nb_instances {
					let position_relative = Vec3::new(
						rand::random::<f32>() * 2.0 - 1.0,
						rand::random::<f32>() * 2.0 - 1.0,
						rand::random::<f32>() * 2.0 - 1.0,
					);
					let position =
						Vec3::from(aabb.center) + Vec3::from(aabb.half_extents) * position_relative;

					ev_spawn_enemy.send(SpawnEnemy {
						pos: position,
						template: assets.enemies[&format!("enemies/{}.enemy.ron", spawner.name)]
							.clone_weak(),
						size: 2f32.powf((rand::random::<f32>() - 0.5) * 2.0),
					});
				}

				cmds.entity(entity).despawn_recursive();
			}
		}
	}
}
