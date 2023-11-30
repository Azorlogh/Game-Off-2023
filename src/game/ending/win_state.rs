use bevy::prelude::*;

use crate::game::GameState;

pub struct WinPlugin;
impl Plugin for WinPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Win), setup_win);
	}
}

fn setup_win(mut cmds: Commands, asset_server: Res<AssetServer>) {
	cmds.spawn(NodeBundle {
		style: Style {
			width: Val::Percent(100.0),
			height: Val::Percent(100.0),
			flex_direction: FlexDirection::Column,
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,

			..default()
		},

		..default()
	})
	.with_children(|cmds| {
		cmds.spawn(NodeBundle {
			style: Style {
				padding: UiRect::all(Val::Px(15.0)),
				flex_direction: FlexDirection::Column,
				justify_content: JustifyContent::FlexStart,
				align_items: AlignItems::Center,
				border: UiRect::all(Val::Px(5.0)),
				..default()
			},
			border_color: BorderColor(Color::WHITE),
			background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
			..default()
		})
		.with_children(|cmds| {
			cmds.spawn(TextBundle::from_section(
				"You win!",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 64.0,
					color: default(),
				},
			));
		});
	});
}
