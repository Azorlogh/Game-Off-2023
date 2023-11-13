use bevy::{core_pipeline::bloom::BloomSettings, math::Vec3Swizzles, prelude::*};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::{
	Collider, CollidingEntities, GravityScale, LockedAxes, RigidBody, Sensor, Velocity,
};

use self::nutrition::{Glucose, Hydration};
use crate::{health::Health, input::Inputs, GameState};

#[derive(Component)]
struct MainCamera;

pub mod nutrition;

const SPEED: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 2.0;
const PLAYER_RADIUS: f32 = 0.5;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<PlayerOnGround>()
			.add_systems(OnExit(GameState::Loading), player_spawn)
			.add_systems(
				Update,
				(
					player_camera,
					player_movement,
					player_on_ground,
					player_jump,
				)
					.run_if(in_state(GameState::Running)),
			);
	}
}

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct PlayerSpawn;

pub fn player_spawn(mut cmds: Commands) {
	cmds.spawn((
		Name::new("Player"),
		Player,
		SpatialBundle::from_transform(Transform::from_xyz(0.0, 10.0, 0.0)),
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
		Health {
			current: 5.0,
			max: 10.0,
		},
		Hydration(0.5),
		Glucose(0.5),
	))
	.with_children(|cmds| {
		cmds.spawn((
			PlayerGroundSensor,
			TransformBundle::from_transform(Transform::from_xyz(0.0, -PLAYER_HEIGHT / 2.0, 0.0)),
			Collider::cylinder(0.2, 0.4),
			Sensor,
			CollidingEntities::default(),
		));
		cmds.spawn((
			Camera3dBundle {
				camera: Camera {
					// hdr: true,
					..default()
				},
				transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0),
				..default()
			},
			MainCamera,
			CameraAngles::default(),
			AtmosphereCamera::default(),
			BloomSettings::default(),
		));
	});
}

#[derive(Default, Component)]
pub struct CameraAngles {
	yaw: f32,
	pitch: f32,
}

fn player_camera(
	inputs: Res<Inputs>,
	mut q_camera: Query<(&mut CameraAngles, &mut Transform), With<MainCamera>>,
) {
	for (mut camera_angles, mut camera_tr) in &mut q_camera {
		camera_angles.yaw += inputs.yaw;
		camera_angles.pitch += inputs.pitch;
		camera_tr.rotation =
			Quat::from_rotation_y(camera_angles.yaw) * Quat::from_rotation_x(camera_angles.pitch);
	}
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

#[derive(Debug, Component, Reflect)]
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
