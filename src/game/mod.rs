use bevy::{gltf::Gltf, prelude::*, utils::HashMap};
use bevy_asset_loader::asset_collection::AssetCollection;

pub(crate) mod enemies;
pub(crate) mod food;
pub(crate) mod hud;
pub(crate) mod level;
pub(crate) mod movement;
pub(crate) mod player;
pub(crate) mod scaling;
pub(crate) mod systems;

use enemies::{template::EnemyTemplate, EnemyPlugin};
use food::FoodPlugin;
use hud::HudPlugin;
use level::LevelPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use systems::*;

use self::scaling::ScalingPlugin;
use crate::AppState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<GameState>()
			.add_plugins((
				MovementPlugin,
				PlayerPlugin,
				FoodPlugin,
				EnemyPlugin,
				HudPlugin,
				LevelPlugin,
				ScalingPlugin,
			))
			.add_systems(Update, toggle_game.run_if(in_state(AppState::Game)))
			.add_systems(OnExit(AppState::Game), despawn_game);
	}
}

// TODO: enlever Menu
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
	#[default]
	None,
	Playing,
	Pause,
	Menu,
}

// Our game's assets
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
	#[asset(key = "world")]
	pub world: Handle<Scene>,
	#[asset(key = "models", collection(typed, mapped))]
	pub models: HashMap<String, Handle<Gltf>>,
	#[asset(key = "enemies", collection(typed, mapped))]
	pub enemies: HashMap<String, Handle<EnemyTemplate>>,
}

#[derive(Component)]
pub struct DespawnOnExitGame;
