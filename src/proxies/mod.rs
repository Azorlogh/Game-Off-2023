//! Some components can't be added directly from GLTF scenes since they don't support reflection
//! So instead, we define our own components that implement Reflect, and later turn them into the ones we want

use bevy::prelude::{App, Plugin};

use self::physics::PhysicsProxies;

pub mod physics;

pub struct GltfProxiesPlugin;
impl Plugin for GltfProxiesPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PhysicsProxies);
	}
}
