use bevy::prelude::*;

use crate::game::scaling::Scaling;

#[derive(Component)]
pub struct Calories(pub f32);

pub fn player_grow(mut q_player: Query<(&mut Scaling, &Calories)>) {
	let Ok((mut scaling, calories)) = q_player.get_single_mut() else {
		return;
	};
	scaling.0 = calories.0 / 1500.0;
}
