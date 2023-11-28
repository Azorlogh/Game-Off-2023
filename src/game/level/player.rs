use bevy::prelude::*;

use crate::game::player::spawn::SpawnPlayer;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct SpawnPoint;

pub fn spawn_player(
	q_added_spawn_points: Query<&Transform, Added<SpawnPoint>>,
	mut ev_spawn_player: EventWriter<SpawnPlayer>,
) {
	for tr in &q_added_spawn_points {
		ev_spawn_player.send(SpawnPlayer(tr.translation));
	}
}
