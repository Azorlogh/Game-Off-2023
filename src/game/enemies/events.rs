use bevy::prelude::*;

use super::template::EnemyTemplate;

#[derive(Event)]
pub struct SpawnEnemy {
	pub template: Handle<EnemyTemplate>,
	pub pos: Vec3,
}
