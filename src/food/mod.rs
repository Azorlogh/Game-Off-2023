use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
	fn build(&self, app: &mut App) {
		app
        .add_systems(Startup, setup_food)
        // .add_systems(Update, food_movement
        ;
	}
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct FoodStats {
	pub hydration: f32,
	pub glucose: f32,
}

impl Default for FoodStats {
	fn default() -> Self {
		Self {
			hydration: 0.2,
			glucose: 0.6,
		}
	}
}

fn setup_food(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		Name::new("Apple"),
		Food,
		FoodStats::default(),
		(RigidBody::Dynamic, Collider::ball(0.5)),
		SceneBundle {
			scene: asset_server.load("models/foods/glb/Apple.glb#Scene0"),
			transform: Transform::from_xyz(0.0, 10.0, 0.0).with_scale(Vec3::splat(0.1)),
			..default()
		},
	));
}
