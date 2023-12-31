use bevy::{
	asset::{AssetLoader, LoadedAsset},
	math::Vec3,
	reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

use super::attack::AttackStats;
use crate::proxies;

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "75ae51b0-8103-4972-b95c-03d3c1cd166d"]
pub struct EnemyTemplate {
	pub model_path: String,
	pub model_scale: f32,
	pub collider: proxies::physics::Collider,
	pub collider_offset: Vec3,
	pub health: f32,
	pub spotting_range: f32,
	pub speed: f32,
	pub attack_stats: AttackStats,
}

pub struct EnemyAssetLoader;

impl AssetLoader for EnemyAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut bevy::asset::LoadContext,
	) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
		Box::pin(async move {
			let asset = ron::de::from_bytes::<EnemyTemplate>(bytes)?;
			load_context.set_default_asset(LoadedAsset::new(asset));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["enemy.ron"]
	}
}
