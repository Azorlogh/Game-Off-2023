use bevy::prelude::*;

use crate::input::Inputs;

/// This component marks the player's camera, which will follow the player eyes entity
/// We don't put the camera as a child of the player, because bevy cameras don't like when they are scaled
#[derive(Component)]
pub struct PlayerCamera;

/// This component is added as a child of the player entity, so that it follows the player's transform incl. scaling
#[derive(Component)]
pub struct PlayerEyes;

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

pub fn camera_follow_eyes(
	q_player_eyes: Query<&GlobalTransform, With<PlayerEyes>>,
	mut q_camera: Query<&mut Transform, &PlayerCamera>,
) {
	let Ok(eyes_tr) = q_player_eyes.get_single() else {
		return;
	};
	let mut camera_tr = q_camera.single_mut();
	camera_tr.translation = eyes_tr.translation();
}
