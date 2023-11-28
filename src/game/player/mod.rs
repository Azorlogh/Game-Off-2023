use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::dynamics::{GravityScale, Velocity};
use eat::player_eat;

use self::{calories::player_grow, camera::PlayerCamera, eat::EatingState, spawn::SpawnPlayer};
use super::{
	movement::{MovementInput, OnGround},
	scaling::Scaling,
};
use crate::{input::Inputs, GameState};

pub mod calories;
pub mod camera;
pub mod eat;
pub mod spawn;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<SpawnPlayer>()
			.insert_resource(EatingState::default())
			.add_systems(
				Update,
				(
					spawn::player_spawn,
					camera::player_camera,
					camera::camera_follow_eyes,
					player_movement,
					player_jump,
					player_eat,
					player_grow,
				)
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
	mut q_player: Query<(&mut Velocity, &mut GravityScale, &OnGround, &Scaling), With<Player>>,
	mut falling: Local<bool>,
) {
	for (mut velocity, mut gravity, on_ground, scaling) in &mut q_player {
		if on_ground.0 && inputs.jump {
			// velocity.linvel.y = 1.0;
			velocity.linvel.y = (6.0 * 9.81 * scaling.0 * scaling.0).sqrt();
			gravity.0 = 2.0 * scaling.0;
			*falling = false;
		} else if !on_ground.0 && !*falling && !inputs.jump {
			gravity.0 = 4.0 * scaling.0;
			*falling = true;
		}
	}
}
