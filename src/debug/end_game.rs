use bevy::prelude::*;

use crate::game::ending::{Lose, Win};

pub fn trigger_end(
	keys: Res<Input<KeyCode>>,
	mut ev_win: EventWriter<Win>,
	mut ev_lose: EventWriter<Lose>,
) {
	if keys.just_pressed(KeyCode::Numpad0) {
		ev_win.send(Win);
	}
	if keys.just_pressed(KeyCode::Numpad1) {
		ev_lose.send(Lose);
	}
}
