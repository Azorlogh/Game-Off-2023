use bevy::prelude::*;

use crate::menu::ColoredButton;

pub const PADDING: f32 = 15.0;
pub const BORDER: f32 = 5.0;

pub fn transparent_root() -> NodeBundle {
	NodeBundle {
		style: Style {
			width: Val::Percent(100.0),
			height: Val::Percent(100.0),
			flex_direction: FlexDirection::Column,
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,

			..default()
		},

		..default()
	}
}

pub fn central_panel() -> NodeBundle {
	NodeBundle {
		style: Style {
			padding: UiRect::all(Val::Px(PADDING)),
			flex_direction: FlexDirection::Column,
			justify_content: JustifyContent::FlexStart,
			align_items: AlignItems::Center,
			border: UiRect::all(Val::Px(BORDER)),
			row_gap: Val::Px(PADDING),
			..default()
		},
		border_color: BorderColor(Color::WHITE),
		background_color: BackgroundColor(Color::ORANGE.with_a(0.5)),
		..default()
	}
}

pub fn button_bundle() -> impl Bundle {
	(
		ButtonBundle {
			style: Style {
				padding: UiRect::all(Val::Px(PADDING)),
				..default()
			},
			border_color: BorderColor(Color::WHITE),
			background_color: BackgroundColor(Color::ORANGE.with_a(0.5)),
			..default()
		},
		ColoredButton,
	)
}

pub fn default_text(text: &str, font_size: f32, asset_server: &AssetServer) -> TextBundle {
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

// pub fn spawn_button<T: Component>(
// 	cmds: &mut ChildBuilder,
// 	asset_server: Res<AssetServer>,
// 	text: &str,
// 	id: Component,
// ) {
// 	cmds.spawn(NodeBundle {
// 		style: Style {
// 			padding: UiRect::all(Val::Px(15.0)),
// 			flex_direction: FlexDirection::Column,
// 			justify_content: JustifyContent::FlexStart,
// 			align_items: AlignItems::Center,
// 			border: UiRect::all(Val::Px(5.0)),
// 			..default()
// 		},
// 		border_color: BorderColor(Color::WHITE),
// 		background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
// 		..default()
// 	})
// 	.with_children(|cmds| {
// 		cmds.spawn(TextBundle::from_section(
// 			"You win!",
// 			TextStyle {
// 				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
// 				font_size: 64.0,
// 				color: default(),
// 			},
// 		));
// 	});
// }
