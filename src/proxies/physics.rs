use bevy::prelude::*;
use bevy_gltf_blueprints::GltfBlueprintsSet;
use bevy_rapier3d::prelude::{
	ActiveEvents, Collider as RapierCollider, ComputedColliderShape, RigidBody,
};

use crate::{util::*, GameState};

pub struct PhysicsProxies;
impl Plugin for PhysicsProxies {
	fn build(&self, app: &mut App) {
		app.register_type::<Collider>().add_systems(
			Update,
			(
				replace_physics_proxies.after(GltfBlueprintsSet::AfterSpawn),
				detach_rigid_bodies,
			).run_if(in_state(GameState::Running)),
		);
	}
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum Collider {
	Ball(f32),
	Cuboid(Vec3),
	Capsule(Vec3, Vec3, f32),
	#[default]
	Mesh,
}

// replaces all physics stand-ins with the actual rapier types
pub fn replace_physics_proxies(
	meshes: Res<Assets<Mesh>>,
	mesh_handles: Query<&Handle<Mesh>>,
	mut proxy_colliders: Query<
		(Entity, &Collider, &Name, &mut Visibility),
		(Without<RapierCollider>, With<Collider>),
	>,
	// needed for tri meshes
	children: Query<&Children>,

	mut commands: Commands,
) {
	for proxy_colider in proxy_colliders.iter_mut() {
		let (entity, collider_proxy, name, mut visibility) = proxy_colider;
		// we hide the collider meshes: perhaps they should be removed altogether once processed ?
		if name.ends_with("_collider") || name.ends_with("_sensor") {
			*visibility = Visibility::Hidden;
		}

		let mut rapier_collider: RapierCollider;
		match collider_proxy {
			Collider::Ball(radius) => {
				rapier_collider = RapierCollider::ball(*radius);
				commands.entity(entity).insert(rapier_collider);
			}
			Collider::Cuboid(size) => {
				rapier_collider = RapierCollider::cuboid(size.x, size.y, size.z);
				commands.entity(entity).insert(rapier_collider);
			}
			Collider::Capsule(a, b, radius) => {
				rapier_collider = RapierCollider::capsule(*a, *b, *radius);
				commands.entity(entity).insert(rapier_collider);
			}
			Collider::Mesh => {
				for (_, collider_mesh) in
					Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
				{
					rapier_collider = RapierCollider::from_bevy_mesh(
						collider_mesh,
						&ComputedColliderShape::ConvexHull,
					)
					.unwrap();
					commands
						.entity(entity)
						.insert(rapier_collider)
						.insert(ActiveEvents::COLLISION_EVENTS);
				}
			}
		}
	}
}

fn detach_rigid_bodies(
	mut cmds: Commands,
	q_added_rigid_bodies: Query<Entity, (With<Parent>, With<RigidBody>)>,
) {
	for entity in &q_added_rigid_bodies {
		cmds.entity(entity).remove_parent();
	}
}
