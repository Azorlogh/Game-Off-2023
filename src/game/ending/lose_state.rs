use bevy::prelude::*;

use crate::{game::GameState, style, AppState};

pub struct LosePlugin;
impl Plugin for LosePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Lose), setup_lose)
			.add_systems(OnExit(GameState::Lose), cleanup_lose)
			.add_systems(Update, (interact_restart, interact_back_to_menu));
	}
}

#[derive(Component)]
pub struct LoseMenu;

#[derive(Component)]
pub struct Restart;

#[derive(Component)]
pub struct BackToMenu;

fn setup_lose(mut cmds: Commands, asset_server: Res<AssetServer>) {
	cmds.spawn((style::transparent_root(), LoseMenu))
		.with_children(|cmds| {
			cmds.spawn(style::central_panel()).with_children(|cmds| {
				cmds.spawn(
					style::default_text("You died :(", 64.0, &asset_server).with_style(Style {
						padding: UiRect::all(Val::Px(style::PADDING)),
						..default()
					}),
				);
				cmds.spawn((style::button_bundle(), Restart))
					.with_children(|cmds| {
						cmds.spawn(style::default_text("Restart", 64.0, &asset_server));
					});
			});
		});
}

fn cleanup_lose(mut cmds: Commands, q_menu: Query<Entity, With<LoseMenu>>) {
	if let Ok(entity) = q_menu.get_single() {
		cmds.entity(entity).despawn_recursive();
	}
}

pub fn interact_restart(
	mut q_button: Query<&Interaction, (Changed<Interaction>, With<Restart>)>,
	mut app_state: ResMut<NextState<AppState>>,
) {
	if let Ok(interaction) = q_button.get_single_mut() {
		match *interaction {
			Interaction::Pressed => {
				app_state.set(AppState::Restart);
			}
			_ => {}
		}
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
