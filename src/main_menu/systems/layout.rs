use bevy::prelude::*;

use crate::main_menu::components::{MainMenuOptions, MainMenuQuit};
use crate::main_menu::styles::*;

use crate::{
	game::player::Player,
	main_menu::components::{MainMenu, MainMenuPlay},
};

pub fn spawn_main_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	q_player: Query<Entity, With<Player>>,
) {
	commands.spawn(Camera2dBundle::default());
	let main_menu_entity = build_main_menu(&mut commands, &asset_server, q_player);
}

pub fn despawn_main_menu(mut commands: Commands, q_main_menu: Query<Entity, With<MainMenu>>) {
	if let Ok(main_menu_entity) = q_main_menu.get_single() {
		commands.entity(main_menu_entity).despawn_recursive();
	}
}

pub fn build_main_menu(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	q_player: Query<Entity, With<Player>>,
) -> Entity {
	println!("Main Menu Opened");
	if let Ok(player_entity) = q_player.get_single() {
		commands.entity(player_entity).despawn_recursive();
	}
	let main_menu_entity = commands
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
				background_color: MAIN_MENU_BACKGROUND_COLOR.into(),
				..default()
			},
			MainMenu {},
		))
		.with_children(|parent| {
			// LOGO
			parent.spawn(ImageBundle {
				style: MAIN_LOGO_STYLE,
				image: asset_server
					.load("sprites/destroy_the_scale_logo.png")
					.into(),
				..default()
			});
			// TITLE
			parent.spawn(default_text("Destroy The Scale !", 64.0, asset_server));
			// PLAY
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					MainMenuPlay {},
				))
				.with_children(|parent| {
					parent.spawn(default_text("Play", 32.0, asset_server));
				});
			// OPTIONS
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					MainMenuOptions {},
				))
				.with_children(|parent| {
					parent.spawn(default_text("Options", 32.0, asset_server));
				});
			// QUIT
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: BUTTON_COLOR.into(),
						..default()
					},
					MainMenuQuit {},
				))
				.with_children(|parent| {
					parent.spawn(default_text("Quit", 32.0, asset_server));
				});
		})
		.id();

	main_menu_entity
}

pub fn default_text(text: &str, font_size: f32, asset_server: &Res<AssetServer>) -> TextBundle {
	TextBundle {
		text: Text {
			sections: vec![TextSection::new(
				text,
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: font_size,
					color: Color::WHITE,
				},
			)],
			alignment: TextAlignment::Center,
			..default()
		},
		..default()
	}
}
