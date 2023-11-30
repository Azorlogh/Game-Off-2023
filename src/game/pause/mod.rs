use bevy::prelude::*;

use super::GameState;
use crate::{
	menu::styles::default_text,
	style::{button_bundle, central_panel, transparent_root},
	AppState,
};

pub struct GamePausePlugin;
impl Plugin for GamePausePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, handle_pause)
			.add_systems(OnEnter(GameState::Pause), setup_pause)
			.add_systems(OnExit(GameState::Pause), cleanup_pause)
			.add_systems(
				Update,
				(interact_resume, interact_back_to_menu).run_if(in_state(GameState::Pause)),
			);
	}
}

fn handle_pause(
	keys: Res<Input<KeyCode>>,
	game_state: Res<State<GameState>>,
	mut next_game_state: ResMut<NextState<GameState>>,
	state: Res<State<GameState>>,
) {
	if keys.just_pressed(KeyCode::Escape)
		&& matches!(game_state.get(), GameState::Playing | GameState::Pause)
	{
		match state.get() {
			GameState::Playing => next_game_state.set(GameState::Pause),
			GameState::Pause => next_game_state.set(GameState::Playing),
			_ => {}
		};
	}
}

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct BackToMenu;

#[derive(Component)]
pub struct Resume;

fn setup_pause(mut cmds: Commands, asset_server: Res<AssetServer>) {
	cmds.spawn((transparent_root(), PauseMenu))
		.with_children(|cmds| {
			cmds.spawn(central_panel()).with_children(|cmds| {
				// Resume
				cmds.spawn((button_bundle(), Resume)).with_children(|cmds| {
					cmds.spawn(default_text("Resume", 32.0, &asset_server));
				});
				// Back to menu
				cmds.spawn((button_bundle(), BackToMenu))
					.with_children(|cmds| {
						cmds.spawn(default_text("Quit to main menu", 32.0, &asset_server));
					});
			});
		});
}

pub fn cleanup_pause(mut cmds: Commands, q_menu: Query<Entity, With<PauseMenu>>) {
	if let Ok(menu) = q_menu.get_single() {
		cmds.entity(menu).despawn_recursive();
	}
}

pub fn interact_back_to_menu(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<BackToMenu>)>,
	mut app_state: ResMut<NextState<AppState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				app_state.set(AppState::MainMenu);
			}
			_ => {}
		}
	}
}

pub fn interact_resume(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Resume>)>,
	mut game_state: ResMut<NextState<GameState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				game_state.set(GameState::Playing);
			}
			_ => {}
		}
	}
}
