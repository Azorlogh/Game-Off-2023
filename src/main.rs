use std::path::PathBuf;

use bevy::{
	audio::AudioPlugin, gltf::Gltf, prelude::*, transform::TransformSystem, utils::HashMap,
};
use bevy_asset_loader::{
	asset_collection::AssetCollection,
	loading_state::{LoadingState, LoadingStateAppExt},
	standard_dynamic_asset::StandardDynamicAssetCollection,
};
use bevy_atmosphere::prelude::{AtmosphereModel, AtmospherePlugin, Nishita};
use bevy_gltf_blueprints::{BlueprintsPlugin, GameWorldTag};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
	prelude::{Collider, NoUserData, PhysicsSet, RapierPhysicsPlugin},
	render::RapierDebugRenderPlugin,
};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_vector_shapes::{painter::ShapeConfig, ShapePlugin};
use input::InputPlugin;
use main_menu::MainMenuPlugin;
use menu::MenuPlugin;
use proxies::GltfProxiesPlugin;
use settings::SettingsPlugin;

use game::enemies::template::EnemyTemplate;
use game::GamePlugin;
mod game;

mod input;
mod main_menu;
mod menu;
mod proxies;
mod settings;
mod util;

fn main() {
	App::new()
		.register_type::<bevy::pbr::wireframe::Wireframe>()
		// External plugins
		.add_plugins((
			DefaultPlugins.build().disable::<AudioPlugin>(), // disabling audio for now because it glitches out on linux when closing the app
			WorldInspectorPlugin::new(),
			BlueprintsPlugin {
				library_folder: PathBuf::from("world/library"),
			},
			// ComponentsFromGltfPlugin::default(),
			RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false),
			GltfProxiesPlugin,
			RapierDebugRenderPlugin::default(),
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
		.configure_sets(
			PostUpdate,
			(
				PhysicsSet::SyncBackend,
				PhysicsSet::SyncBackendFlush,
				PhysicsSet::StepSimulation,
				PhysicsSet::Writeback,
			)
				.chain()
				.before(TransformSystem::TransformPropagate),
		)
		.add_systems(
			PostUpdate,
			(
				RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend)
					.in_set(PhysicsSet::SyncBackend),
				RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackendFlush)
					.in_set(PhysicsSet::SyncBackendFlush),
				RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation)
					.in_set(PhysicsSet::StepSimulation),
				RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback)
					.in_set(PhysicsSet::Writeback),
			), //.run_if(in_state(GameState::Running).or_else(in_state(GameState::Loading)))
		)
		// Once the assets are loaded, spawn the level
		.add_systems(OnExit(GameState::Loading), spawn_level)
		.run();
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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
	#[default]
	Loading,
	Running,
	Menu,
	Pause,
	MainMenu,
}
const SUN_POSITION: Vec3 = Vec3::new(3.0, 10.0, 4.0);

fn spawn_level(
	mut commands: Commands,
	game_assets: Res<GameAssets>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		SceneBundle {
			scene: game_assets.world.clone(),
			..default()
		},
		GameWorldTag,
	));

	commands.insert_resource(AtmosphereModel::new(Nishita {
		sun_position: SUN_POSITION,
		..default()
	}));

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 10000.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::default().looking_to(-SUN_POSITION, Vec3::Y),
		..default()
	});

	let ground_size = 200.1;
	let ground_height = 0.1;
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Box::new(
				ground_size * 2.0,
				ground_height * 2.0,
				ground_size * 2.0,
			))),
			material: materials.add(Color::rgb_u8(124, 144, 255).into()),
			transform: Transform::from_xyz(0.0, -ground_height - 5.0, 0.0),
			..default()
		},
		Collider::cuboid(ground_size, ground_height, ground_size),
	));
}

// Disabled because it breaks animations
// fn show_full_entity_names(mut q_names: Query<(Entity, &mut Name), Added<Name>>) {
// 	for (entity, mut name) in q_names.iter_mut() {
// 		name.mutate(|name| *name += &format!(" ({entity:?})"));
// 	}
// }
