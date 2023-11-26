use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::dynamics::{GravityScale, Velocity};
use eat::player_eat;

use self::{
	camera::{player_camera, PlayerCamera},
	spawn::player_spawn,
};
use super::movement::{MovementInput, OnGround};
use crate::{input::Inputs, AppState, GameState};

pub mod camera;
pub mod eat;
pub mod nutrition;
pub mod spawn;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::Game), player_spawn)
			.add_systems(
				Update,
				(player_camera, player_movement, player_jump, player_eat)
					.run_if(in_state(GameState::Playing)),
			);
	}
}

#[derive(Component)]
pub struct Player;

pub fn player_movement(
	inputs: Res<Inputs>,
	mut q_player: Query<&mut MovementInput, With<Player>>,
	q_camera: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
) {
	for mut movement_input in &mut q_player {
		let camera_tr = q_camera.single();

		let camera_forward = (camera_tr.forward() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
		let camera_right = (camera_tr.right() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
		let dir = (camera_forward * inputs.dir.y + camera_right * inputs.dir.x).xz();

		movement_input.0 = dir;
	}
}

pub fn player_jump(
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
