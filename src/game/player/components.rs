use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Default, Component)]
pub struct CameraAngles {
	pub yaw: f32,
	pub pitch: f32,
}
