use bevy::prelude::*;

use super::{Enemy, EnemyState};
use crate::game::movement::MovementInput;

pub struct EnemyRoamingPlugin;
impl Plugin for EnemyRoamingPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, enemy_roam);
	}
}

#[derive(Reflect)]
pub enum RoamingState {
	Waiting { remaining: f32 },
	GoingTo { dir: Vec2, remaining: f32 },
}

pub fn enemy_roam(
	time: Res<Time>,
	mut q_enemies: Query<(&mut EnemyState, &mut MovementInput), With<Enemy>>,
) {
	for (mut enemy_state, mut input) in &mut q_enemies {
		let EnemyState::Roaming(state) = enemy_state.as_mut() else {
			continue;
		};

		match state {
			RoamingState::Waiting { remaining } => {
				input.0 = Vec2::ZERO;
				let new_remaining = *remaining - time.delta_seconds();
				if new_remaining <= 0.0 {
					let dir = (Vec2::new(rand::random(), rand::random()) - 0.5).normalize_or_zero();
					*state = RoamingState::GoingTo {
						dir,
						remaining: rand::random::<f32>() * 4.0 + 2.0,
					};
				} else {
					*state = RoamingState::Waiting {
						remaining: new_remaining,
					}
				}
			}
			RoamingState::GoingTo { dir, remaining } => {
				input.0 = *dir * 1.0; // 0.5: walking speed
				let new_remaining = *remaining - time.delta_seconds();
				if new_remaining <= 0.0 {
					*state = RoamingState::Waiting {
						remaining: rand::random::<f32>() * 4.0 + 4.0,
					}
				} else {
					*state = RoamingState::GoingTo {
						dir: *dir,
						remaining: new_remaining,
					};
				}
			}
		}
	}
}
