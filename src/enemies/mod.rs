mod model;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use self::model::EnemyModelPlugin;
use crate::player::Player;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnemyModelPlugin)
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

#[derive(Component)]
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

fn enemy_start_chase(
	q_player: Query<&GlobalTransform, With<Player>>,
	mut q_enemies: Query<(&mut EnemyState, &GlobalTransform), With<Enemy>>,
) {
	for (enemy_state, enemy_tr) in &mut q_enemies {}
}
