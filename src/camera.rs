use std::f32::consts::TAU;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_dolly::prelude::*;

use crate::{input::Inputs, player::Player};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup).add_systems(
			PostUpdate,
			(
				camera_offset,
				camera_follow,
				Dolly::<MainCamera>::update_active.in_set(DollyUpdateSet),
			)
				.chain(),
		);
	}
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Component)]
pub struct CameraOffset {
	pub pitch: f32,
	pub yaw: f32,
}

fn setup(mut cmds: Commands) {
	cmds.spawn((
		Camera3dBundle {
			camera: Camera {
				hdr: true,
				..default()
			},
			..default()
		},
		MainCamera,
		AtmosphereCamera::default(),
		CameraOffset::default(),
		BloomSettings::default(),
		Rig::builder()
			.with(Position::new(Vec3::ZERO))
			.with(YawPitch::new().pitch_degrees(-90.0))
			.with(Smooth::new_rotation(0.5))
			.with(Arm::new(Vec3::Z * 10.0))
			.build(),
	));
}

fn camera_offset(
	time: Res<Time>,
	inputs: Res<Inputs>,
	mut q_camera_offset: Query<&mut CameraOffset>,
) {
	let mut cam_offset = q_camera_offset.single_mut();
	cam_offset.pitch =
		(cam_offset.pitch - inputs.pitch * time.delta_seconds() * 4.0).rem_euclid(TAU);
	cam_offset.yaw = (cam_offset.yaw - inputs.yaw * time.delta_seconds() * 4.0).rem_euclid(TAU);
}

fn camera_follow(
	mut q_camera: Query<(&mut Rig, &CameraOffset), With<MainCamera>>,
	q_player: Query<&Transform, With<Player>>,
) {
	let Ok((mut rig, cam_offset)) = q_camera.get_single_mut() else {
		return;
	};

	let Ok(player_tr) = q_player.get_single() else {
		return;
	};

	rig.driver_mut::<Position>().position = player_tr.translation + Vec3::Y * 2.0;
	rig.driver_mut::<YawPitch>().yaw_degrees = cam_offset.yaw.to_degrees();
	rig.driver_mut::<YawPitch>().pitch_degrees = -25.0 - cam_offset.pitch.to_degrees();
}
