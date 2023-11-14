mod model;

use bevy::{
	math::{Vec3Swizzles, Vec4Swizzles},
	prelude::*,
};
use bevy_rapier3d::{
	dynamics::{LockedAxes, Velocity},
	prelude::{Collider, RigidBody},
};

use self::model::EnemyModelPlugin;
use crate::{health::Health, player::Player};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnemyModelPlugin)
			.register_type::<EnemyState>()
			.add_event::<SpawnEnemy>()
			.add_systems(Startup, setup)
			.add_systems(Update, (enemy_spawn, enemy_start_chase, enemy_chase));
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

impl EnemyState {
	pub fn chasing_target(&self) -> Option<Entity> {
		match self {
			EnemyState::Idle => None,
			EnemyState::Attack(target) => Some(*target),
		}
	}
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
			SpatialBundle::from_transform(
				Transform::from_translation(ev.pos).with_scale(Vec3::splat(0.5)),
			),
			RigidBody::Dynamic,
			EnemyState::Idle,
			Velocity::default(),
			LockedAxes::ROTATION_LOCKED,
			Health {
				current: 5.0,
				max: 10.0,
			},
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

const ENEMY_SPEED: f32 = 7.0;

fn enemy_chase(
	time: Res<Time>,
	q_global_transform: Query<&GlobalTransform>,
	mut q_enemies: Query<(&EnemyState, (Entity, &mut Transform, &mut Velocity))>,
) {
	for (target, (enemy_entity, mut enemy_tr, mut vel)) in q_enemies
		.iter_mut()
		.filter_map(|(state, comp)| state.chasing_target().map(|t| (t, comp)))
	{
		let enemy_gtr = q_global_transform.get(enemy_entity).unwrap();
		let target_gtr = q_global_transform.get(target).unwrap();
		let to_target_dir = (target_gtr.translation() - enemy_gtr.translation()).normalize()
			* Vec3::new(1.0, 0.0, 1.0);

		let axis = enemy_gtr.forward().cross(to_target_dir);
		enemy_tr.rotate(Quat::from_scaled_axis(axis * 0.1));

		enemy_tr.translation += to_target_dir * ENEMY_SPEED * time.delta_seconds();

		let linvel = vel.linvel;
		vel.linvel += (to_target_dir * -(to_target_dir.xz().dot(linvel.xz()).max(0.0)))
			.extend(0.0)
			.xzy();
	}
}
