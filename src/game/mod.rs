use bevy::{
	gltf::Gltf,
	prelude::*,
	utils::HashMap,
	window::{CursorGrabMode, PrimaryWindow},
};
use bevy_asset_loader::asset_collection::AssetCollection;

pub(crate) mod endboss;
pub(crate) mod ending;
pub(crate) mod enemies;
pub(crate) mod food;
pub(crate) mod health;
pub(crate) mod hit_effect;
pub(crate) mod hud;
pub(crate) mod level;
pub(crate) mod movement;
pub(crate) mod pause;
pub(crate) mod player;
pub(crate) mod scaling;

use bevy_rapier3d::dynamics::RigidBody;
use enemies::{template::EnemyTemplate, EnemyPlugin};
use food::FoodPlugin;
use hud::HudPlugin;
use level::LevelPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

use self::{
	endboss::EndbossPlugin, ending::GameEndPlugin, health::HealthPlugin,
	hit_effect::HitEffectPlugin, pause::GamePausePlugin, scaling::ScalingPlugin,
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
				GamePausePlugin,
				HitEffectPlugin,
			))
			.add_systems(OnExit(AppState::Game), despawn_game)
			.add_systems(OnExit(GameState::Playing), exit_playing)
			.add_systems(Update, mark_game_entities);
	}
}

// TODO: enlever Menu
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
	#[default]
	None,
	Playing,
	Pause,
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
			"world/library/Croissant.glb",
			"world/library/Eggplant.glb",
			// "world/library/Foo.glb",
			// "world/library/Hotdog.glb",
			"world/library/Orange.glb",
			// "world/library/Steak.glb",
			"world/library/Tomato.glb",
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

pub fn despawn_game(
	mut commands: Commands,
	q_game_despawn: Query<Entity, With<DespawnOnExitGame>>,
	q_food_despawn: Query<Entity, With<food::components::Food>>,
) {
	for game_entity in q_game_despawn.iter() {
		commands.entity(game_entity).despawn_recursive();
	}

	for food_entity in q_food_despawn.iter() {
		commands.entity(food_entity).despawn_recursive();
	}
}

fn mark_game_entities(mut cmds: Commands, q_entities: Query<Entity, Added<RigidBody>>) {
	for entity in &q_entities {
		cmds.entity(entity).insert(DespawnOnExitGame);
	}
}

fn exit_playing(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
	let mut window = q_window.single_mut();
	window.cursor.grab_mode = CursorGrabMode::None;
	window.cursor.visible = true;
}
