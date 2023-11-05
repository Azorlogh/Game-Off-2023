use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::{
	Collider, CollidingEntities, GravityScale, LockedAxes, RigidBody, Sensor, Velocity,
};

use crate::{camera::MainCamera, input::Inputs, GameState};

const SPEED: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 2.0;
const PLAYER_RADIUS: f32 = 0.5;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Running), player_spawn)
			.add_systems(Update, (player_movement, player_on_ground, player_jump));
	}
}

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct PlayerSpawn;

pub fn player_spawn(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	cmds.spawn((
		Player,
		PbrBundle {
			mesh: meshes.add(
				shape::Capsule {
					radius: PLAYER_RADIUS,
					depth: PLAYER_HEIGHT - PLAYER_RADIUS * 2.0,
					..default()
				}
				.into(),
			),
			material: materials.add(StandardMaterial {
				base_color: Color::RED,
				..default()
			}),
			transform: Transform::from_xyz(0.0, 10.0, 0.0),
			..default()
		},
		RigidBody::Dynamic,
		Velocity::default(),
		Collider::capsule(
			Vec3::Y * -(PLAYER_HEIGHT / 2.0 - PLAYER_RADIUS),
			Vec3::Y * (PLAYER_HEIGHT / 2.0 - PLAYER_RADIUS),
			PLAYER_RADIUS,
		),
		LockedAxes::ROTATION_LOCKED,
		CollidingEntities::default(),
		PlayerOnGround(false),
		GravityScale(2.0),
	))
	.with_children(|cmds| {
		cmds.spawn((
			PlayerGroundSensor,
			TransformBundle::from_transform(Transform::from_xyz(0.0, -PLAYER_HEIGHT / 2.0, 0.0)),
			Collider::cylinder(0.2, 0.4),
			Sensor,
			CollidingEntities::default(),
		));
	});
}

fn player_movement(
	time: Res<Time>,
	inputs: Res<Inputs>,
	mut q_player: Query<(&mut Transform, &mut Velocity), With<Player>>,
	q_camera: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
	for (mut player_tr, mut vel) in &mut q_player {
		let camera_tr = q_camera.single();

		let camera_forward = (camera_tr.forward() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
		let camera_right = (camera_tr.right() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
		let dir = camera_forward * inputs.dir.y + camera_right * inputs.dir.x;
		player_tr.translation += dir * SPEED * time.delta_seconds();
		let linvel = vel.linvel;
		vel.linvel += (inputs.dir * -(inputs.dir.dot(linvel.xz()).max(0.0)))
			.extend(0.0)
			.xzy();
	}
}

#[derive(Component)]
pub struct PlayerGroundSensor;

#[derive(Debug, Component)]
pub struct PlayerOnGround(bool);

fn player_on_ground(
	q_sensor: Query<&CollidingEntities, With<PlayerGroundSensor>>,
	mut q_player: Query<&mut PlayerOnGround>,
) {
	for mut on_ground in &mut q_player {
		for sensor in &q_sensor {
			on_ground.0 = !sensor.is_empty();
		}
	}
}

fn player_jump(
	inputs: Res<Inputs>,
	mut q_player: Query<(&mut Velocity, &mut GravityScale, &PlayerOnGround), With<Player>>,
	mut falling: Local<bool>,
) {
	for (mut velocity, mut gravity, on_ground) in &mut q_player {
		if on_ground.0 && inputs.jump {
			velocity.linvel.y = 7.0;
			gravity.0 = 1.0;
			*falling = false;
		} else if !on_ground.0 && !*falling && !inputs.jump {
			gravity.0 = 2.0;
			*falling = true;
		}
	}
}
