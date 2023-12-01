use std::path::PathBuf;

use bevy::{
	app::App, audio::AudioPlugin, ecs::schedule::SystemConfigs, prelude::*, DefaultPlugins,
};
use bevy_asset_loader::{
	loading_state::{LoadingState, LoadingStateAppExt},
	standard_dynamic_asset::StandardDynamicAssetCollection,
};
#[cfg(not(target_arch = "wasm32"))]
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
use menu::MainMenuPlugin;
use proxies::GltfProxiesPlugin;
use settings::SettingsPlugin;
mod game;

mod debug;
mod input;
mod menu;
mod proxies;
mod settings;
mod style;
mod util;

const DEBUG: bool = true;

fn main() {
	let mut app = App::new();
	app.register_type::<bevy::pbr::wireframe::Wireframe>()
		// External plugins
		.add_plugins((
			DefaultPlugins
				.set(bevy::window::WindowPlugin {
					primary_window: Some(Window {
						fit_canvas_to_parent: true,
						..default()
					}),
					..default()
				})
				.set(bevy::log::LogPlugin {
					level: bevy::log::Level::INFO,
					..default()
				})
				.build()
				.disable::<AudioPlugin>(), // disabling audio for now because it glitches out on linux when csing the app
			BlueprintsPlugin {
				library_folder: PathBuf::from("world/library"),
			},
			// ComponentsFromGltfPlugin::default(),
			RapierPhysicsPlugin::<NoUserData>::default(), //.with_default_system_setup(false),
			RapierDebugRenderPlugin {
				enabled: DEBUG,
				..default()
			},
			GltfProxiesPlugin,
			#[cfg(not(target_arch = "wasm32"))]
			AtmospherePlugin,
			ShapePlugin {
				base_config: ShapeConfig {
					alignment: bevy_vector_shapes::shapes::Alignment::Billboard,
					..ShapeConfig::default_3d()
				},
				..default()
			},
		))
		// Our own plugins
		.add_plugins((GamePlugin, MainMenuPlugin, InputPlugin, SettingsPlugin))
		// Game state
		.add_state::<AppState>()
		.add_state::<GameState>()
		.add_loading_state(
			LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu),
		)
		// Game assets: Tell our app to load the assets from GameAssets
		.add_collection_to_loading_state::<_, GameAssets>(AppState::Loading)
		.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
			AppState::Loading,
			"assets_game.assets.ron",
		)
		.add_systems(OnEnter(AppState::Game), transition_to(GameState::Playing))
		.add_systems(OnExit(AppState::Game), transition_to(GameState::None))
		.add_systems(OnEnter(AppState::Restart), transition_to(AppState::Game));

	if DEBUG {
		app.add_plugins((
			WorldInspectorPlugin::new(),
			DebugPlugin,
			ScreenDiagnosticsPlugin::default(),
			ScreenFrameDiagnosticsPlugin,
		));
	}

	app.run();
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
	#[default]
	Loading,
	MainMenu,
	Game,
	Restart,
}

fn transition_to<S: States + Copy>(s: S) -> SystemConfigs {
	IntoSystemConfigs::into_configs(move |mut next_state: ResMut<NextState<S>>| next_state.set(s))
}
