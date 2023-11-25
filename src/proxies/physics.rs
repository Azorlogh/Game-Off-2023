use bevy::prelude::*;
use bevy_gltf_blueprints::GltfBlueprintsSet;
use bevy_rapier3d::prelude::{
	ActiveEvents, Collider as RapierCollider, ComputedColliderShape, RigidBody,
};
use serde::Deserialize;

use crate::util::*;

pub struct PhysicsProxies;
impl Plugin for PhysicsProxies {
	fn build(&self, app: &mut App) {
		app.register_type::<Collider>().add_systems(
			Update,
			(
				replace_physics_proxies.after(GltfBlueprintsSet::AfterSpawn),
				detach_rigid_bodies,
				fix_buggy_scale_issue,
			),
		);
	}
}

#[derive(Clone, Component, Reflect, Default, Debug, Deserialize)]
#[reflect(Component)]
pub enum Collider {
	Ball(f32),
	Cuboid(Vec3),
	Capsule(Vec3, Vec3, f32),
	Cylinder(f32, f32),
	#[default]
	Mesh,
	MeshConcave,
}

// replaces all physics stand-ins with the actual rapier types
pub fn replace_physics_proxies(
	meshes: Res<Assets<Mesh>>,
	mesh_handles: Query<&Handle<Mesh>>,
	mut proxy_colliders: Query<(Entity, &Collider), (Without<RapierCollider>, With<Collider>)>,
	// needed for tri meshes
	children: Query<&Children>,

	mut commands: Commands,
) {
	for proxy_colider in proxy_colliders.iter_mut() {
		let (entity, collider_proxy) = proxy_colider;

		let mut rapier_collider: RapierCollider;
		commands.entity(entity).insert(MustFixBuggyScale(2));
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
			Collider::Cylinder(a, radius) => {
				rapier_collider = RapierCollider::cylinder(*a, *radius);
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
			Collider::MeshConcave => {
				for (_, collider_mesh) in
					Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
				{
					rapier_collider = RapierCollider::from_bevy_mesh(
						collider_mesh,
						&ComputedColliderShape::ConvexDecomposition(default()),
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

// Scale is initially f-ed up for some reason, so a system will reset it after a few frames
#[derive(Component)]
pub struct MustFixBuggyScale(i16);

fn fix_buggy_scale_issue(mut q_objs: Query<(&mut MustFixBuggyScale, &mut Transform)>) {
	for (mut mfbs, mut transform) in &mut q_objs {
		mfbs.0 -= 1;
		if mfbs.0 <= 0 {
			transform.set_changed();
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
