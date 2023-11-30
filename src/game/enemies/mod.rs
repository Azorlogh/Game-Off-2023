//!
//! TODO: Let 'em jump?
//!

mod model;
mod roaming;
pub mod template;

use bevy::prelude::*;

use self::{
	attack::AttackState,
	model::EnemyModelPlugin,
	roaming::{EnemyRoamingPlugin, RoamingState},
	spawn::SpawnEnemy,
	template::{EnemyAssetLoader, EnemyTemplate},
};
use super::movement::MovementInput;
use crate::AppState;

pub mod attack;
pub mod spawn;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((EnemyModelPlugin, EnemyRoamingPlugin))
			.register_type::<EnemyState>()
			.add_asset::<EnemyTemplate>()
			.add_asset_loader(EnemyAssetLoader)
			.add_event::<SpawnEnemy>()
			.add_systems(OnEnter(AppState::Game), spawn::setup)
			.add_systems(Update, spawn::enemy_spawn)
			.add_systems(
				Update,
				(
					attack::enemy_start_chase,
					attack::enemy_chase,
					attack::enemy_attack,
					align_to_walking_dir,
				),
			);
	}
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Reflect)]
pub enum EnemyState {
	Roaming(RoamingState),
	Attacking(Entity, AttackState),
}

pub fn align_to_walking_dir(
	mut q_enemies: Query<(&mut Transform, &GlobalTransform, &mut MovementInput)>,
) {
	for (mut enemy_tr, enemy_gtr, enemy_input) in &mut q_enemies {
		let dir = enemy_input.0.normalize_or_zero();
		let axis = enemy_gtr.forward().cross(Vec3::new(dir.x, 0.0, dir.y));
		enemy_tr.rotate(Quat::from_scaled_axis(axis * 0.1));
	}
}
