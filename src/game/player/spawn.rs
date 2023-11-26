use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_rapier3d::{
	dynamics::{CoefficientCombineRule, GravityScale, LockedAxes, RigidBody, Velocity},
	geometry::{Collider, CollidingEntities, Friction, Restitution},
};

use super::{
	camera::{CameraAngles, PlayerCamera, PlayerEyes},
	nutrition::{Glucose, Hydration},
	Player,
};
use crate::game::{
	hud::health::{Health, HideHealthBar},
	movement::{GroundSensorBundle, MovementInput, OnGround, Speed},
	scaling::Scaling,
	DespawnOnExitGame,
};

const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_RADIUS: f32 = 0.25;
const PLAYER_EYE_OFFSET: f32 = (PLAYER_HEIGHT * 0.92) / 2.0; // relative to center of body

#[derive(Event)]
pub struct PlayerSpawn;

pub fn player_spawn(mut cmds: Commands) {
	cmds.spawn((
		Name::new("Player"),
		Player,
		DespawnOnExitGame,
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
			Scaling(0.2),
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
			transform: Transform::from_xyz(0.0, PLAYER_EYE_OFFSET, 0.0),
			projection: Projection::Perspective(PerspectiveProjection {
				fov: std::f32::consts::PI / 4.0 * 1.5,
				near: 0.01,
				..default()
			}),
			..default()
		},
		PlayerCamera,
		CameraAngles::default(),
		AtmosphereCamera::default(),
		BloomSettings::default(),
	));
}
