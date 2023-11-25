use bevy::{
	core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
	math::Vec3Swizzles,
	prelude::*,
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::{
	dynamics::CoefficientCombineRule,
	geometry::{Friction, Restitution},
	prelude::{Collider, CollidingEntities, GravityScale, LockedAxes, RigidBody, Velocity},
};
use eat::player_eat;
use nutrition::{Glucose, Hydration};

use crate::{
	health::{Health, HideHealthBar},
	input::Inputs,
	movement::{GroundSensorBundle, MovementInput, OnGround, Speed},
	scaling::Scaling,
	GameState,
};

#[derive(Component)]
pub struct MainCamera;

pub mod eat;
pub mod nutrition;

const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_RADIUS: f32 = 0.25;
const PLAYER_EYE_OFFSET: f32 = (PLAYER_HEIGHT * 0.92) / 2.0; // relative to center of body

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnExit(GameState::Loading), player_spawn)
			.add_systems(
				Update,
				(
					player_camera,
					player_movement,
					player_jump,
					player_eat,
					camera_follow_eyes,
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
		SpatialBundle::from_transform(Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0)),
		(
			RigidBody::Dynamic,
			Velocity::default(),
			Collider::capsule(
				Vec3::Y * -(PLAYER_HEIGHT / 2.0 - PLAYER_RADIUS),
				Vec3::Y * (PLAYER_HEIGHT / 2.0 - PLAYER_RADIUS),
				PLAYER_RADIUS,
			),
			LockedAxes::ROTATION_LOCKED,
			CollidingEntities::default(),
			GravityScale(2.0),
			Friction {
				coefficient: 0.0,
				combine_rule: CoefficientCombineRule::Min,
			},
			Restitution {
				coefficient: 0.0,
				combine_rule: CoefficientCombineRule::Min,
			},
		),
		(
			OnGround(false),
			MovementInput::default(),
			Speed(10.0),
			Scaling(1.0),
		),
		(
			Health {
				current: 100,
				max: 100,
			},
			Hydration(0),
			Glucose(0),
			HideHealthBar,
		),
	))
	.with_children(|cmds| {
		cmds.spawn(GroundSensorBundle::new(
			PLAYER_RADIUS * 0.8,
			-PLAYER_HEIGHT / 2.0,
		));
		cmds.spawn((
			PlayerEyes,
			TransformBundle::from_transform(Transform::from_xyz(0.0, PLAYER_EYE_OFFSET, 0.0)),
		));
	});
	cmds.spawn((
		Camera3dBundle {
			camera: Camera {
				hdr: true,
				..default()
			},
			projection: Projection::Perspective(PerspectiveProjection {
				fov: std::f32::consts::PI / 4.0 * 1.5,
				near: 0.01,
				..default()
			}),
			..default()
		},
		MainCamera,
		CameraAngles::default(),
		AtmosphereCamera::default(),
		BloomSettings {
			intensity: 0.05,
			..default()
		},
	));
}

#[derive(Component)]
pub struct PlayerEyes;

fn camera_follow_eyes(
	q_player_eyes: Query<&GlobalTransform, With<PlayerEyes>>,
	mut q_camera: Query<&mut Transform, &MainCamera>,
) {
	let Ok(eyes_tr) = q_player_eyes.get_single() else {
		return;
	};
	let mut camera_tr = q_camera.single_mut();
	camera_tr.translation = eyes_tr.translation();
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
	inputs: Res<Inputs>,
	mut q_player: Query<&mut MovementInput, With<Player>>,
	q_camera: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
	for mut movement_input in &mut q_player {
		let camera_tr = q_camera.single();

		let camera_forward = (camera_tr.forward() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
		let camera_right = (camera_tr.right() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
		let dir = (camera_forward * inputs.dir.y + camera_right * inputs.dir.x).xz();

		movement_input.0 = dir;
	}
}

fn player_jump(
	inputs: Res<Inputs>,
	mut q_player: Query<(&mut Velocity, &mut GravityScale, &OnGround), With<Player>>,
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
