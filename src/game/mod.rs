use bevy::{gltf::Gltf, prelude::*, utils::HashMap};
use bevy_asset_loader::asset_collection::AssetCollection;

pub(crate) mod endboss;
pub(crate) mod ending;
pub(crate) mod enemies;
pub(crate) mod food;
pub(crate) mod health;
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

use self::{
	endboss::EndbossPlugin, ending::GameEndPlugin, health::HealthPlugin, scaling::ScalingPlugin,
};
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
				HealthPlugin,
				EndbossPlugin,
				GameEndPlugin,
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
	Win,
	Lose,
}

// Our game's assets
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
	#[asset(key = "world")]
	pub world: Handle<Scene>,
	// #[asset(key = "models", collection(typed, mapped))]
	#[asset(
		paths(
			// "world/library/Apple.glb",
			// "world/library/Avocado.glb",
			// "world/library/Banana.glb",
			// "world/library/Bread_Slice.glb",
			// "world/library/Broccoli.glb",
			"world/library/Burger.glb",
			// "world/library/Carrot.glb",
			// "world/library/ChickenLeg.glb",
			// "world/library/Croissant.glb",
			// "world/library/Eggplant.glb",
			// "world/library/Foo.glb",
			// "world/library/Hotdog.glb",
			// "world/library/Orange.glb",
			// "world/library/Steak.glb",
			// "world/library/Tomato.glb",
			"world/library/BreadCrumb.glb",
			"world/library/CornKernel.glb",
		),
		collection(typed, mapped)
	)]
	pub models: HashMap<String, Handle<Gltf>>,
	// #[asset(key = "enemies", collection(typed, mapped))]
	#[asset(
		paths(
			"enemies/rat.enemy.ron",
			"enemies/snake.enemy.ron",
			"enemies/spider.enemy.ron",
		),
		collection(typed, mapped)
	)]
	pub enemies: HashMap<String, Handle<EnemyTemplate>>,
}

#[derive(Component)]
pub struct DespawnOnExitGame;
