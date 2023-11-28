use bevy::prelude::*;

use crate::game::{player::Player, scaling::Scaling};

pub fn scale(mut q_player: Query<&mut Scaling, With<Player>>, keys: Res<Input<KeyCode>>) {
	if keys.just_pressed(KeyCode::I) {
		let mut scaling = q_player.single_mut();
		scaling.0 *= 1.2;
	}
	if keys.just_pressed(KeyCode::O) {
		let mut scaling = q_player.single_mut();
		scaling.0 /= 1.2;
	}
}
