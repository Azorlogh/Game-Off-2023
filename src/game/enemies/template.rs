use bevy::{
	asset::{io::Reader, Asset, AssetLoader, AsyncReadExt},
	reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;
use thiserror::Error;

use super::attack::AttackStats;

#[derive(Debug, Deserialize, TypeUuid, TypePath, Asset)]
#[uuid = "75ae51b0-8103-4972-b95c-03d3c1cd166d"]
pub struct EnemyTemplate {
	pub model_path: String,
	pub scale: f32,
	pub health: u32,
	pub spotting_range: f32,
	pub speed: f32,
	pub attack_stats: AttackStats,
}

// TODO: just use anyhow::Error when 0.12.1 lands
#[derive(Debug, Error)]
pub enum EnemyAssetLoaderError {
	#[error("Could not read the file: {0}")]
	Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct EnemyAssetLoader;

impl AssetLoader for EnemyAssetLoader {
	type Asset = EnemyTemplate;
	type Settings = ();
	type Error = EnemyAssetLoaderError;

	fn load<'a>(
		&'a self,
		reader: &'a mut Reader,
		_settings: &'a Self::Settings,
		load_context: &'a mut bevy::asset::LoadContext,
	) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
		Box::pin(async move {
			let mut bytes = Vec::new();
			reader.read_to_end(&mut bytes).await?;
			let asset = ron::de::from_bytes::<EnemyTemplate>(&bytes).unwrap();
			Ok(asset)
		})
	}

	fn extensions(&self) -> &[&str] {
		&["enemy.ron"]
	}
}
