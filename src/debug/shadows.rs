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
