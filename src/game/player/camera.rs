use bevy::prelude::*;

use crate::input::Inputs;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Default, Component)]
pub struct CameraAngles {
	pub yaw: f32,
	pub pitch: f32,
}

pub fn player_camera(
	inputs: Res<Inputs>,
	mut q_camera: Query<(&mut CameraAngles, &mut Transform), With<PlayerCamera>>,
) {
	for (mut camera_angles, mut camera_tr) in &mut q_camera {
		camera_angles.yaw += inputs.yaw;
		camera_angles.pitch += inputs.pitch;
		camera_tr.rotation =
			Quat::from_rotation_y(camera_angles.yaw) * Quat::from_rotation_x(camera_angles.pitch);
	}
}
