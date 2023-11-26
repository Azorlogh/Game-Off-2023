use bevy::prelude::*;
use bevy_atmosphere::{collection::nishita::Nishita, model::AtmosphereModel};
use bevy_gltf_blueprints::GameWorldTag;
use bevy_rapier3d::geometry::Collider;

use crate::{
	game::{DespawnOnExitGame, GameAssets},
	AppState,
};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		// Once the assets are loaded, spawn the level
		app.add_systems(OnEnter(AppState::Game), spawn_level);
	}
}

const SUN_POSITION: Vec3 = Vec3::new(3.0, 10.0, 4.0);

pub fn spawn_level(
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
		DespawnOnExitGame,
	));

	commands.insert_resource(AtmosphereModel::new(Nishita {
		sun_position: SUN_POSITION,
		..default()
	}));

	commands.spawn((
		DirectionalLightBundle {
			directional_light: DirectionalLight {
				illuminance: 10000.0,
				shadows_enabled: true,
				..default()
			},
			transform: Transform::default().looking_to(-SUN_POSITION, Vec3::Y),
			..default()
		},
		DespawnOnExitGame,
	));

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
		DespawnOnExitGame,
		Collider::cuboid(ground_size, ground_height, ground_size),
	));
	info!("Level Spawned!");
}
