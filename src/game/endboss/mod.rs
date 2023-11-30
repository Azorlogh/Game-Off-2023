use bevy::prelude::*;

use super::{ending::Win, health::Dead};

pub struct EndbossPlugin;
impl Plugin for EndbossPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, trigger_win_on_boss_destroyed);
	}
}

#[derive(Component)]
pub struct EndBoss;

// When the scale is destroyed
fn trigger_win_on_boss_destroyed(
	q_boss: Query<Entity, (With<EndBoss>, With<Dead>)>,
	mut ev_win: EventWriter<Win>,
) {
	if q_boss.get_single().is_ok() {
		ev_win.send(Win);
	}
}
