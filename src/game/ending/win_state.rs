use bevy::prelude::*;

use crate::{game::GameState, style};

pub struct WinPlugin;
impl Plugin for WinPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Win), setup_win);
	}
}

fn setup_win(mut cmds: Commands, asset_server: Res<AssetServer>) {
	cmds.spawn(style::transparent_root()).with_children(|cmds| {
		cmds.spawn(style::central_panel()).with_children(|cmds| {
			cmds.spawn(
				style::default_text("You Won!", 64.0, &asset_server).with_style(Style {
					padding: UiRect::all(Val::Px(style::PADDING)),
					..default()
				}),
			);
			cmds.spawn(
				style::default_text(
					"You have managed to destroy your dangerous invention.",
					32.0,
					&asset_server,
				)
				.with_style(Style {
					padding: UiRect::all(Val::Px(style::PADDING)),
					..default()
				}),
			);
		});
	});
}
