use std::path::PathBuf;

use bevy::{app::App, audio::AudioPlugin, prelude::*, DefaultPlugins};
use bevy_asset_loader::{
	loading_state::{LoadingState, LoadingStateAppExt},
	standard_dynamic_asset::StandardDynamicAssetCollection,
};
use bevy_atmosphere::prelude::AtmospherePlugin;
use bevy_gltf_blueprints::BlueprintsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
	prelude::{NoUserData, RapierPhysicsPlugin},
	render::RapierDebugRenderPlugin,
};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_vector_shapes::{painter::ShapeConfig, ShapePlugin};
use debug::DebugPlugin;
use game::{GameAssets, GamePlugin, GameState};
use input::InputPlugin;
use main_menu::MainMenuPlugin;
use menu::MenuPlugin;
use proxies::GltfProxiesPlugin;
use settings::SettingsPlugin;
use systems::{enter_game, quit_game, transition_to_game_state, transition_to_main_menu_state};
mod game;

mod debug;
mod input;
mod main_menu;
mod menu;
mod proxies;
mod settings;
mod systems;
mod util;

const DEBUG: bool = true;

fn main() {
	let mut app = App::new();
	app.register_type::<bevy::pbr::wireframe::Wireframe>()
		// External plugins
		.add_plugins((
			DefaultPlugins
				.set(bevy::log::LogPlugin {
					level: bevy::log::Level::INFO,
					..default()
				})
				.build()
				.disable::<AudioPlugin>(), // disabling audio for now because it glitches out on linux when csing the app
			WorldInspectorPlugin::new(),
			BlueprintsPlugin {
				library_folder: PathBuf::from("world/library"),
			},
			// ComponentsFromGltfPlugin::default(),
			RapierPhysicsPlugin::<NoUserData>::default(), //.with_default_system_setup(false),
			GltfProxiesPlugin,
			AtmospherePlugin,
			ShapePlugin {
				base_config: ShapeConfig {
					alignment: bevy_vector_shapes::shapes::Alignment::Billboard,
					..ShapeConfig::default_3d()
				},
				..default()
			},
			ScreenDiagnosticsPlugin::default(),
			ScreenFrameDiagnosticsPlugin,
		))
		// Our own plugins
		.add_plugins((
			GamePlugin,
			MainMenuPlugin,
			InputPlugin,
			SettingsPlugin,
			MenuPlugin,
			DebugPlugin,
		))
		// Game state
		.add_state::<AppState>()
		.add_state::<GameState>()
		.add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::Game))
		// Game assets: Tell our app to load the assets from GameAssets
		.add_collection_to_loading_state::<_, GameAssets>(AppState::Loading)
		.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
			AppState::Loading,
			"assets_game.assets.ron",
		)
		// Systems
		.add_systems(
			Update,
			(transition_to_game_state, transition_to_main_menu_state),
		)
		.add_systems(OnEnter(AppState::Game), enter_game)
		.add_systems(OnExit(AppState::Game), quit_game);

	if DEBUG {
		app.add_plugins(RapierDebugRenderPlugin::default());
	}

	app.run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
	#[default]
	Loading,
	MainMenu,
	Game,
	GameOver,
}
