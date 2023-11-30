use bevy::prelude::*;
use bevy_rapier3d::{dynamics::ExternalImpulse, pipeline::QueryFilter, plugin::RapierContext};

use super::{camera::PlayerCamera, Player};
use crate::{
	game::{health::Hit, scaling::Scaling},
	input::Inputs,
};

#[derive(Debug, Default, Resource)]
pub enum PunchingState {
	#[default]
	Idle,
	Punching(
		/// Time until the punch lands
		f32,
	),
}

const PUNCHING_RANGE: f32 = 3.0;

pub fn player_punch(
	mut state: ResMut<PunchingState>,
	time: Res<Time>,
	inputs: Res<Inputs>,
	rapier_context: Res<RapierContext>,
	q_player: Query<(Entity, &Scaling), With<Player>>,
	q_player_camera: Query<&GlobalTransform, With<PlayerCamera>>,
	mut q_external_impulse: Query<&mut ExternalImpulse>,
	q_parent: Query<&Parent>,
	mut ev_hit: EventWriter<Hit>,
) {
	match &*state {
		PunchingState::Idle if inputs.punch => {
			*state = PunchingState::Punching(1.0);
		}
		PunchingState::Punching(remaining) => {
			let new_remaining = remaining - time.delta_seconds() / 0.3;
			if new_remaining <= 0.0 {
				// punch lands
				let Ok((player_entity, scaling)) = q_player.get_single() else {
					return;
				};
				let Ok(player_camera_tr) = q_player_camera.get_single() else {
					return;
				};

				let filter: QueryFilter = QueryFilter::new().exclude_rigid_body(player_entity);
				if let Some((entity, _toi)) = rapier_context.cast_ray(
					player_camera_tr.translation(),
					player_camera_tr.forward(),
					PUNCHING_RANGE * scaling.0,
					true,
					filter,
				) {
					debug!("Hit something");
					let mut target_entity = q_external_impulse.contains(entity).then_some(entity);
					if target_entity.is_none() {
						for ancestor in q_parent.iter_ancestors(entity) {
							if q_external_impulse.contains(ancestor) {
								target_entity = Some(ancestor);
							}
						}
					}
					if let Some((target_entity, mut impulse)) = target_entity
						.and_then(|e| q_external_impulse.get_mut(e).ok().map(|imp| (e, imp)))
					{
						debug!("Hit something that can receive a hit");
						let mut dir = player_camera_tr.forward();
						dir.y = 0.1;
						impulse.impulse += dir.normalize_or_zero() * 0.001 * scaling.0;
						ev_hit.send(Hit {
							target: target_entity,
							damage: 10.0 * scaling.0,
						});
					}
				}
				*state = PunchingState::Idle;
			} else {
				*state = PunchingState::Punching(new_remaining);
			}
		}
		_ => {}
	}
}
