use std::{f32::consts::TAU, time::Duration};

use bevy::{prelude::*, utils::HashMap};

use super::{Enemy, EnemyState};

pub struct EnemyModelPlugin;
impl Plugin for EnemyModelPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(add_enemy_models, enemy_animate, enemy_update_animation),
		);
	}
}

#[derive(Component)]
pub struct AnimationPlayerLink(Entity);

#[derive(Debug, PartialEq, Eq, Hash, Component)]
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
					transform: Transform::from_rotation(Quat::from_rotation_y(TAU / 2.0)),
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
				AnimationState::Idle,
			));
		});
	}
}

fn enemy_animate(
	mut cmds: Commands,
	q_model_animations: Query<(Entity, &ModelAnimations)>,
	q_parent: Query<&Parent>,
	mut q_added_animation_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
	for (anim_player_entity, mut anim_player) in &mut q_added_animation_players {
		if let Ok((model_entity, animations)) = q_parent
			.get(anim_player_entity)
			.and_then(|p| q_parent.get(p.get()))
			.and_then(|p| q_model_animations.get(p.get()))
		{
			cmds.entity(model_entity)
				.insert(AnimationPlayerLink(anim_player_entity));
			anim_player
				.play(animations.0[&AnimationState::Idle].clone_weak())
				.repeat();
		}
	}
}

fn enemy_update_animation(
	q_enemies: Query<&EnemyState>,
	mut q_models: Query<(
		&Parent,
		&AnimationPlayerLink,
		&mut AnimationState,
		&ModelAnimations,
	)>,
	mut q_anim_player: Query<&mut AnimationPlayer>,
) {
	for (parent, anim_player_link, mut anim_state, anims) in &mut q_models {
		let enemy_state = q_enemies.get(parent.get()).unwrap();

		let mut anim_player = q_anim_player.get_mut(anim_player_link.0).unwrap();

		let new_anim_state = match enemy_state {
			EnemyState::Idle => AnimationState::Idle,
			EnemyState::Attack(_) => AnimationState::Run,
		};
		println!("{:?}", new_anim_state);

		if new_anim_state != *anim_state {
			*anim_state = new_anim_state;
			anim_player
				.play_with_transition(
					anims.0[&*anim_state].clone_weak(),
					Duration::from_millis(200),
				)
				.repeat();
		}
	}
}
