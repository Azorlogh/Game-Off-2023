use bevy::{audio::AudioPlugin, gltf::Gltf, prelude::*, utils::HashMap};
use bevy_asset_loader::{
	asset_collection::AssetCollection,
	loading_state::{LoadingState, LoadingStateAppExt},
	standard_dynamic_asset::StandardDynamicAssetCollection,
};
use bevy_atmosphere::prelude::AtmospherePlugin;
use bevy_rapier3d::{
	prelude::{NoUserData, RapierPhysicsPlugin},
	render::RapierDebugRenderPlugin,
};
use input::InputPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use proxies::GltfProxiesPlugin;
use ::{
	bevy_gltf_blueprints::{BlueprintsPlugin, GameWorldTag},
	bevy_inspector_egui::quick::WorldInspectorPlugin,
};

mod input;
mod player;
mod proxies;
mod util;
mod menu;

fn main() {
	App::new()
		.register_type::<bevy::pbr::wireframe::Wireframe>()
		// External plugins
		.add_plugins((
			DefaultPlugins.build().disable::<AudioPlugin>(), // disabling audio for now because it glitches out on linux when closing the app
			WorldInspectorPlugin::new(),
			BlueprintsPlugin::default(),
			GltfProxiesPlugin,
			RapierPhysicsPlugin::<NoUserData>::default(),
			RapierDebugRenderPlugin::default(),
			AtmospherePlugin,
		))
		// Our own plugins
		.add_plugins((InputPlugin, PlayerPlugin, MenuPlugin))
		// Game state
		.add_state::<GameState>()
		.add_loading_state(
			LoadingState::new(GameState::Loading).continue_to_state(GameState::Charging),
		)
		// Game assets: Tell our app to load the assets from GameAssets
		.add_collection_to_loading_state::<_, GameAssets>(GameState::Loading)
		.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
			GameState::Loading,
			"assets_game.assets.ron",
		)
		// Once the assets are loaded, spawn the level
		.add_systems(OnEnter(GameState::Charging), (spawn_level, change_state))
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
	Charging,
	Running,
	Menu,
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

fn change_state(mut app_state: ResMut<NextState<GameState>>) {
	app_state.set(GameState::Running);
}