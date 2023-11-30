use bevy::prelude::*;

use crate::{game::GameState, style};

pub struct LosePlugin;
impl Plugin for LosePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Lose), setup_lose);
	}
}

fn setup_lose(mut cmds: Commands, asset_server: Res<AssetServer>) {
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
				padding: UiRect::all(Val::Px(style::PADDING)),
				flex_direction: FlexDirection::Column,
				justify_content: JustifyContent::FlexStart,
				align_items: AlignItems::Center,
				border: UiRect::all(Val::Px(style::BORDER)),
				..default()
			},
			border_color: BorderColor(Color::WHITE),
			background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
			..default()
		})
		.with_children(|cmds| {
			cmds.spawn(
				TextBundle::from_section(
					"You lost :(",
					TextStyle {
						font: asset_server.load("fonts/FiraSans-Bold.ttf"),
						font_size: 64.0,
						color: default(),
					},
				)
				.with_style(Style {
					padding: UiRect::all(Val::Px(style::PADDING)),
					..default()
				}),
			);
			cmds.spawn(style::button_bundle()).with_children(|cmds| {
				cmds.spawn(TextBundle::from_section(
					"Restart",
					TextStyle {
						font: asset_server.load("fonts/FiraSans-Bold.ttf"),
						font_size: 64.0,
						color: default(),
					},
				));
			});
			// style::spawn_button(cmds, asset_server, "Restart");
		});
	});
}
