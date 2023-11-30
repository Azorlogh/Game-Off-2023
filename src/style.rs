use bevy::prelude::*;

pub const PADDING: f32 = 15.0;
pub const BORDER: f32 = 5.0;

pub fn button_bundle() -> ButtonBundle {
	ButtonBundle {
		style: Style {
			padding: UiRect::all(Val::Px(PADDING)),
			..default()
		},
		border_color: BorderColor(Color::WHITE),
		background_color: BackgroundColor(Color::ORANGE.with_a(0.5)),
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
