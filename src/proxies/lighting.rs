use bevy::{
	pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder},
	prelude::*,
};

pub struct LightingProxiesPlugin;
impl Plugin for LightingProxiesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, lighting_replace_proxies);
	}
}

pub fn lighting_replace_proxies(
	mut added_dirights: Query<(Entity, &mut DirectionalLight), Added<DirectionalLight>>,
	mut added_spotlights: Query<&mut SpotLight, Added<SpotLight>>,
	mut added_pointlights: Query<&mut PointLight, Added<PointLight>>,

	mut commands: Commands,
) {
	for (entity, mut light) in added_dirights.iter_mut() {
		// light.illuminance *= 5.0;
		light.shadows_enabled = false;
		let shadow_config: CascadeShadowConfig = CascadeShadowConfigBuilder {
			first_cascade_far_bound: 15.0,
			maximum_distance: 135.0,
			..default()
		}
		.into();

		commands.entity(entity).insert(shadow_config);
	}
	for mut light in added_spotlights.iter_mut() {
		light.intensity *= 0.0025;
		light.shadows_enabled = false;
	}

	for mut light in added_pointlights.iter_mut() {
		light.intensity *= 0.0025; // arbitrary/ eyeballed to match the levels of Blender
		light.color = Color::RgbaLinear {
			red: light.color.r(),
			green: light.color.g(),
			blue: light.color.b(),
			alpha: 1.0,
		}
		.as_rgba();
		light.shadows_enabled = false;
	}
}
