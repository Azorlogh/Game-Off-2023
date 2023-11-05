use bevy::{gltf::Gltf, prelude::*, utils::HashMap};
use bevy_asset_loader::{
	asset_collection::AssetCollection,
	loading_state::{LoadingState, LoadingStateAppExt},
	standard_dynamic_asset::StandardDynamicAssetCollection,
};
use bevy_gltf_blueprints::{BlueprintsPlugin, GameWorldTag};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
	App::new()
		.register_type::<bevy::pbr::wireframe::Wireframe>()
		.add_plugins((
			DefaultPlugins,
			WorldInspectorPlugin::new(),
			BlueprintsPlugin::default(),
		))
		// Game state
		.add_state::<GameState>()
		.add_loading_state(
			LoadingState::new(GameState::Loading).continue_to_state(GameState::Running),
		)
		// Game assets: Tell our app to load the assets from GameAssets
		.add_collection_to_loading_state::<_, GameAssets>(GameState::Loading)
		.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
			GameState::Loading,
			"assets_game.assets.ron",
		)
		// Once the assets are loaded, spawn the level
		.add_systems(OnEnter(GameState::Running), spawn_level)
		.add_systems(Startup, setup)
		.run();
}

// Our game's assets
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
	#[asset(key = "world")]
	pub world: Handle<Scene>,
	#[asset(key = "models", collection(typed, mapped))]
	pub models: HashMap<String, Handle<Gltf>>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
	#[default]
	Loading,
	Running,
}

fn spawn_level(mut commands: Commands, game_assets: Res<GameAssets>) {
	commands.spawn((
		SceneBundle {
			scene: game_assets.world.clone(),
			..default()
		},
		GameWorldTag,
	));
}

fn setup(mut commands: Commands) {
	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
	// camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}
