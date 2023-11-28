//! Some components can't be added directly from GLTF scenes since they don't support reflection
//! So instead, we define our own components that implement Reflect, and later turn them into the ones we want

use bevy::prelude::{App, Plugin};

use self::{
	lighting::LightingProxiesPlugin, physics::PhysicsProxies, random_model::RandomModelPlugin,
};

pub mod lighting;
pub mod physics;
mod random_model;

pub struct GltfProxiesPlugin;
impl Plugin for GltfProxiesPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((PhysicsProxies, LightingProxiesPlugin, RandomModelPlugin));
	}
}
