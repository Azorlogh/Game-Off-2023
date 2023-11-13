use bevy::{prelude::*, utils::HashMap};

use super::Enemy;

pub struct EnemyModelPlugin;
impl Plugin for EnemyModelPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (add_enemy_models, enemy_animate));
	}
}

#[derive(PartialEq, Eq, Hash)]
pub enum AnimationState {
	Idle,
	Run,
}

#[derive(Component)]
pub struct ModelAnimations(HashMap<AnimationState, Handle<AnimationClip>>);

fn add_enemy_models(
	mut cmds: Commands,
	q_added_enemies: Query<Entity, Added<Enemy>>,
	asset_server: Res<AssetServer>,
) {
	for entity in &q_added_enemies {
		cmds.entity(entity).with_children(|cmds| {
			cmds.spawn((
				SceneBundle {
					scene: asset_server.load("models/characters/rat.gltf#Scene0"),
					..default()
				},
				ModelAnimations(
					[
						(
							AnimationState::Idle,
							asset_server.load("models/characters/rat.gltf#Animation2"),
						),
						(
							AnimationState::Run,
							asset_server.load("models/characters/rat.gltf#Animation4"),
						),
					]
					.into_iter()
					.collect(),
				),
			));
		});
	}
}

fn enemy_animate(
	q_model_animations: Query<&ModelAnimations>,
	q_parent: Query<&Parent>,
	mut q_added_animation_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
	for (anim_player_entity, mut anim_player) in &mut q_added_animation_players {
		if let Ok(animations) = q_parent
			.get(anim_player_entity)
			.and_then(|p| q_parent.get(p.get()))
			.and_then(|p| q_model_animations.get(p.get()))
		{
			anim_player
				.play(animations.0[&AnimationState::Run].clone_weak())
				.repeat();
		}
	}
}
