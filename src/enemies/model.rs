use std::{f32::consts::TAU, time::Duration};

use bevy::{prelude::*, utils::HashMap};

use super::{template::EnemyTemplate, Enemy, EnemyState};
use crate::enemies::AttackState;

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
	Attack,
}

#[derive(Component)]
pub struct ModelAnimations(HashMap<AnimationState, Handle<AnimationClip>>);

fn add_enemy_models(
	mut cmds: Commands,
	q_added_enemies: Query<(Entity, &Handle<EnemyTemplate>), Added<Enemy>>,
	enemy_assets: Res<Assets<EnemyTemplate>>,
	asset_server: Res<AssetServer>,
) {
	for (entity, template) in &q_added_enemies {
		let model_path = &enemy_assets.get(template).unwrap().model_path;

		cmds.entity(entity).with_children(|cmds| {
			cmds.spawn((
				SceneBundle {
					scene: asset_server.load(format!("{model_path}#Scene0")),
					transform: Transform::from_rotation(Quat::from_rotation_y(TAU / 2.0)),
					..default()
				},
				ModelAnimations(
					[
						(
							AnimationState::Idle,
							asset_server.load(format!("{model_path}#Animation2")),
						),
						(
							AnimationState::Run,
							asset_server.load(format!("{model_path}#Animation4")),
						),
						(
							AnimationState::Attack,
							asset_server.load(format!("{model_path}#Animation0")),
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
			EnemyState::Attacking(_, AttackState::Chasing) => AnimationState::Run,
			EnemyState::Attacking(_, AttackState::Attacking(_)) => AnimationState::Attack,
		};

		if new_anim_state != *anim_state {
			*anim_state = new_anim_state;
			anim_player
				.play_with_transition(
					anims.0[&*anim_state].clone_weak(),
					Duration::from_millis(200),
				)
				.repeat()
				.set_speed(match *anim_state {
					AnimationState::Run => 2.0,
					AnimationState::Idle => 1.0,
					AnimationState::Attack => 0.45,
				});
		}
	}
}
