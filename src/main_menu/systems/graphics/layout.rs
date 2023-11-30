use bevy::prelude::*;

use crate::{
	game::player::Player,
	main_menu::{styles::*, ColoredButton},
};

use super::{GraphicsBack, GraphicsMenu};

pub fn spawn_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	q_player: Query<Entity, With<Player>>,
) {
	let _option_menu_entity = build_menu(&mut commands, &asset_server, q_player);
}

pub fn despawn_menu(mut commands: Commands, q_menu: Query<Entity, With<GraphicsMenu>>) {
	if let Ok(menu_entity) = q_menu.get_single() {
		commands.entity(menu_entity).despawn_recursive();
	}
}

pub fn build_menu(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	q_player: Query<Entity, With<Player>>,
) -> Entity {
	if let Ok(player_entity) = q_player.get_single() {
		commands.entity(player_entity).despawn_recursive();
	}
	let menu_entity = commands
		.spawn((
			NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Column,
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					row_gap: Val::Px(8.0),
					..default()
				},
				background_color: DEFAULT_BACKGROUND_COLOR.into(),
				..default()
			},
			GraphicsMenu,
		))
		.with_children(|parent| {
			// BACK
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					GraphicsBack,
					ColoredButton,
				))
				.with_children(|parent| {
					parent.spawn(default_text("Back", 32.0, asset_server));
				});
		})
		.id();

	menu_entity
}
