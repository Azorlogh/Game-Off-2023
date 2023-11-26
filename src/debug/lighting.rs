use bevy::prelude::*;

pub fn toggle_shadows(
	mut q_dirights: Query<&mut DirectionalLight>,
	mut q_spotlights: Query<&mut SpotLight>,
	mut q_pointlights: Query<&mut PointLight>,
	mut disabled: Local<bool>,
	keys: Res<Input<KeyCode>>,
) {
	if keys.just_pressed(KeyCode::L) {
		*disabled = !*disabled;
		for mut light in &mut q_dirights {
			light.shadows_enabled = !*disabled;
		}
		for mut light in &mut q_spotlights {
			light.shadows_enabled = !*disabled;
		}
		for mut light in &mut q_pointlights {
			light.shadows_enabled = !*disabled;
		}
	}
}

pub fn despawn_lights(
	mut cmds: Commands,
	mut q_lights: Query<Entity, With<SpotLight>>,
	mut disabled: Local<bool>,
	keys: Res<Input<KeyCode>>,
) {
	if keys.just_pressed(KeyCode::K) {
		*disabled = !*disabled;
		for entity in &mut q_lights {
			cmds.entity(entity).despawn_recursive();
		}
	}
}
