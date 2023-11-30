use bevy::prelude::*;
use bevy_rapier3d::render::DebugRenderContext;

pub fn toggle_debug(keys: Res<Input<KeyCode>>, mut debug: ResMut<DebugRenderContext>) {
	if keys.just_pressed(KeyCode::U) {
		debug.enabled = !debug.enabled;
	}
}
