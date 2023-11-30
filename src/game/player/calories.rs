use bevy::prelude::*;

use crate::game::scaling::Scaling;

#[derive(Component)]
pub struct Calories(pub f32);

impl Calories {
	pub fn to_scaling(&self) -> f32 {
		self.0 / 1500.0
	}
}

pub fn player_grow(mut q_player: Query<(&mut Scaling, &Calories)>) {
	let Ok((mut scaling, calories)) = q_player.get_single_mut() else {
		return;
	};
	scaling.0 = calories.to_scaling();
}
