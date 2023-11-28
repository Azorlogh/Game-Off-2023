use bevy::prelude::*;

use crate::game::player::{calories::Calories, Player};

pub fn scale(mut q_player: Query<&mut Calories, With<Player>>, keys: Res<Input<KeyCode>>) {
	if keys.just_pressed(KeyCode::I) {
		let mut calories = q_player.single_mut();
		calories.0 *= 1.2;
	}
	if keys.just_pressed(KeyCode::O) {
		let mut calories = q_player.single_mut();
		calories.0 /= 1.2;
	}
}
