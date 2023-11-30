use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	prelude::*,
	render::{camera::ScalingMode, view::RenderLayers},
};

mod crosshair;
pub(crate) mod player_ui;

use player_ui::*;

use self::crosshair::CrosshairPlugin;
use super::DespawnOnExitGame;
use crate::AppState;

pub struct HudPlugin;
impl Plugin for HudPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((CrosshairPlugin, PlayerUiPlugin))
			.add_systems(OnEnter(AppState::Game), setup);
	}
}

fn setup(mut cmds: Commands) {
	cmds.spawn((
		Camera2dBundle {
			camera: Camera {
				order: 1,
				hdr: true,
				..default()
			},
			camera_2d: Camera2d {
				clear_color: ClearColorConfig::None,
			},
			projection: OrthographicProjection {
				scaling_mode: ScalingMode::FixedVertical(2.0),
				..default()
			},
			..default()
		},
		RenderLayers::layer(1),
		DespawnOnExitGame,
	));
}
