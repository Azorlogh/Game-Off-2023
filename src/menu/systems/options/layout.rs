use bevy::prelude::*;

use super::{Graphics, Keyboard, OptionsBack, OptionsMenu, Sounds};
use crate::{
	game::player::Player,
	menu::{styles::*, ColoredButton},
};

pub fn spawn_option_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	q_player: Query<Entity, With<Player>>,
) {
	let _option_menu_entity = build_option_menu(&mut commands, &asset_server, q_player);
}

pub fn despawn_option_menu(mut commands: Commands, q_menu: Query<Entity, With<OptionsMenu>>) {
	if let Ok(menu_entity) = q_menu.get_single() {
		commands.entity(menu_entity).despawn_recursive();
	}
}

pub fn build_option_menu(
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
			OptionsMenu,
		))
		.with_children(|parent| {
			// GRAPHICS
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					Graphics,
					ColoredButton,
				))
				.with_children(|parent| {
					parent.spawn(default_text("Graphics", 32.0, asset_server));
				});
			// KEYBOARD
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					Keyboard,
					ColoredButton,
				))
				.with_children(|parent| {
					parent.spawn(default_text("Keyboard", 32.0, asset_server));
				});
			// SOUNDS
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					Sounds,
					ColoredButton,
				))
				.with_children(|parent| {
					parent.spawn(default_text("Sound", 32.0, asset_server));
				});
			// BACK
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					OptionsBack,
					ColoredButton,
				))
				.with_children(|parent| {
					parent.spawn(default_text("Back", 32.0, asset_server));
				});
		})
		.id();

	menu_entity
}
