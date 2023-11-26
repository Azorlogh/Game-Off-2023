//!
//! TODO: Let 'em jump?
//!

mod model;
pub mod template;

use bevy::prelude::*;

use self::{
	attack::AttackState,
	model::EnemyModelPlugin,
	spawn::SpawnEnemy,
	template::{EnemyAssetLoader, EnemyTemplate},
};
use crate::AppState;

pub mod attack;
pub mod spawn;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnemyModelPlugin)
			.register_type::<EnemyState>()
			.init_asset::<EnemyTemplate>()
			.init_asset_loader::<EnemyAssetLoader>()
			.add_event::<SpawnEnemy>()
			.add_systems(OnEnter(AppState::Game), spawn::setup)
			.add_systems(Update, spawn::enemy_spawn)
			.add_systems(
				Update,
				(
					attack::enemy_start_chase,
					attack::enemy_chase,
					attack::enemy_attack,
				),
			);
	}
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Reflect)]
pub enum EnemyState {
	Idle,
	Attacking(Entity, AttackState),
}
