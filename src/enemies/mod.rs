mod model;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use self::model::EnemyModelPlugin;
use crate::player::Player;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnemyModelPlugin)
			.register_type::<EnemyState>()
			.add_event::<SpawnEnemy>()
			.add_systems(Startup, setup)
			.add_systems(Update, (enemy_spawn, enemy_start_chase));
	}
}

#[derive(Event)]
pub struct SpawnEnemy {
	pos: Vec3,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Reflect)]
pub enum EnemyState {
	Idle,
	Attack(Entity),
}

fn setup(mut ev_spawn_enemy: EventWriter<SpawnEnemy>) {
	ev_spawn_enemy.send(SpawnEnemy {
		pos: Vec3::new(0.0, 0.0, -5.0),
	})
}

fn enemy_spawn(mut cmds: Commands, mut ev_spawn_enemy: EventReader<SpawnEnemy>) {
	for ev in ev_spawn_enemy.iter() {
		cmds.spawn((
			Name::new("Enemy"),
			Enemy,
			SpatialBundle::from_transform(Transform::from_translation(ev.pos)),
			RigidBody::Dynamic,
			EnemyState::Idle,
		))
		.with_children(|cmds| {
			cmds.spawn((
				TransformBundle::from_transform(Transform::from_translation(Vec3::Y * 1.0)),
				Collider::cuboid(1.0, 1.0, 2.0),
			));
		});
	}
}

const ENEMY_VIEW_DISTANCE: f32 = 4.0;

fn enemy_start_chase(
	q_player: Query<(Entity, &GlobalTransform), With<Player>>,
	mut q_enemies: Query<(&mut EnemyState, &GlobalTransform), With<Enemy>>,
) {
	for (mut enemy_state, enemy_tr) in q_enemies
		.iter_mut()
		.filter(|(state, _)| matches!(**state, EnemyState::Idle))
	{
		for (player_entity, player_tr) in &q_player {
			if enemy_tr.translation().distance(player_tr.translation()) < ENEMY_VIEW_DISTANCE {
				*enemy_state = EnemyState::Attack(player_entity);
			}
		}
	}
}

// fn enemy_chase(
// 	q_global_transform: Query<&GlobalTransform>,
// 	q_enemies: Query<&EnemyState>,
// ) {
// 	for enemy_state in q_enemies.iter().map(||)
// }
