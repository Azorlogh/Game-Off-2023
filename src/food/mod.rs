use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::LinePainter};

use crate::player::MainCamera;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_food)
			.add_systems(
				Update,
				(
					// spawn_food,
					display_health_food,
				),
			)
			.add_event::<SpawnFood>()
			.register_type::<FoodStats>()
			.register_type::<FoodProperties>()
			.register_type::<Food>();
	}
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Food;

#[derive(Event)]
pub struct SpawnFood {
	pub name: String,
	pub model: String,
	pub stats: FoodStats,
	pub position: Vec3,
	pub scale_factor: f32,
	pub properties: FoodProperties,
}

#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct FoodStats {
	pub hydration: i32,
	pub glucose: i32,
	pub fat: i32,
	pub health: i32,
}
impl Default for FoodStats {
	fn default() -> Self {
		Self {
			hydration: 1,
			glucose: 1,
			fat: 1,
			health: 0,
		}
	}
}

#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct FoodProperties {
	pub health: u32,
	pub total_health: u32,
	pub time_per_bite: f32,
}

fn setup_food(mut ev_spawn_food: EventWriter<SpawnFood>) {
	ev_spawn_food.send(SpawnFood {
		name: String::from("Apple"),
		model: String::from("models/foods/glb/Apple.glb#Scene0"),
		stats: FoodStats {
			hydration: 3,
			glucose: 7,
			fat: 2,
			health: 4,
		},
		properties: FoodProperties {
			health: 3,
			total_health: 3,
			time_per_bite: 1.0,
		},
		position: Vec3::new(0.0, 10.0, 0.0),
		scale_factor: 0.1,
	})
}

#[allow(unused)]
fn spawn_food(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut event_reader: EventReader<SpawnFood>,
) {
	for event in event_reader.iter() {
		commands.spawn((
			Name::new(event.name.clone()),
			Food,
			event.properties,
			event.stats,
			(RigidBody::Dynamic, Collider::ball(0.5)),
			SceneBundle {
				scene: asset_server.load(event.model.clone()),
				transform: Transform::from_translation(event.position)
					.with_scale(Vec3::splat(event.scale_factor)),
				..default()
			},
		));
	}
}

fn display_health_food(
	mut painter: ShapePainter,
	query: Query<(&FoodProperties, &GlobalTransform), With<Food>>,
	q_camera: Query<&GlobalTransform, With<MainCamera>>,
) {
	const HEALTHBAR_LENGTH: f32 = 0.25;
	let Ok(camera_tr) = q_camera.get_single() else {
		return;
	};

	for (food_properties, transform) in &query {
		painter.thickness = 0.02;
		painter.color = Color::GRAY;
		let healthbar_pos = transform.translation() + Vec3::Y * 0.1;
		let healthbar_left = healthbar_pos - camera_tr.right() * HEALTHBAR_LENGTH / 2.0;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * HEALTHBAR_LENGTH,
		);

		let health_ratio = food_properties.health as f32 / food_properties.total_health as f32;

		painter.color = Color::ORANGE;
		painter.line(
			healthbar_left,
			healthbar_left + camera_tr.right() * (HEALTHBAR_LENGTH * health_ratio),
		);
	}
}
