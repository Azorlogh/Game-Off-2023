use bevy::{prelude::*, render::primitives::Aabb};
use bevy_gltf_blueprints::{BlueprintName, SpawnHere};

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlueprintSpawner {
	blueprints: Vec<String>,
	density: f32,
}

pub fn spawn_blueprints(
	mut cmds: Commands,
	q_added_blueprints_spawners: Query<(Entity, &BlueprintSpawner)>,
	q_children: Query<&Children>,
	mut q_aabb: Query<&Aabb>,
) {
	for (entity, spawner) in &q_added_blueprints_spawners {
		for descendant_e in q_children.iter_descendants(entity) {
			if let Ok(aabb) = q_aabb.get_mut(descendant_e) {
				let sides = aabb.half_extents * 2.0;
				let nb_instances = (sides.x * sides.y * sides.z * spawner.density) as usize;
				println!("Spawning {} instances!", nb_instances);
				for _ in 0..nb_instances {
					let position_relative = Vec3::new(
						rand::random::<f32>() * 2.0 - 1.0,
						rand::random::<f32>() * 2.0 - 1.0,
						rand::random::<f32>() * 2.0 - 1.0,
					);
					let bp_index = rand::random::<usize>() % spawner.blueprints.len();
					let bp_name = spawner.blueprints[bp_index].clone();
					cmds.spawn((
						Name::new(format!("{:?} instance", bp_name)),
						BlueprintName(bp_name),
						SpawnHere,
						SpatialBundle::from_transform(Transform::from_translation(
							Vec3::from(aabb.center)
								+ Vec3::from(aabb.half_extents) * position_relative,
						)),
					));
				}

				cmds.entity(entity).despawn_recursive();
			}
		}
	}
}
