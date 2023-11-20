use bevy::prelude::*;

use serde::Deserialize;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct SpottingRange(pub f32);

#[derive(Debug, Clone, Component, Deserialize)]
pub struct AttackStats {
	pub range: f32,
	pub speed: f32,
	pub damage: u32,
}

#[derive(Clone, Copy, Reflect)]
pub enum AttackState {
	Chasing,
	Attacking(f32),
}

#[derive(Component, Reflect)]
pub enum EnemyState {
	Idle,
	Attacking(Entity, AttackState),
}
