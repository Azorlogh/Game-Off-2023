use bevy::prelude::*;

pub const MAIN_MENU_BACKGROUND_COLOR: Color = Color::ORANGE;
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::ORANGE;

pub const BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const BUTTON_WIDTH: f32 = 200.0;
pub const BUTTON_HEIGHT: f32 = 80.0;

pub const BUTTON_STYLE: Style = {
	let mut style = Style::DEFAULT;
	style.width = Val::Px(BUTTON_WIDTH);
	style.height = Val::Px(BUTTON_HEIGHT);
	style.justify_content = JustifyContent::Center;
	style.align_items = AlignItems::Center;
	style
};

pub const MAIN_LOGO_STYLE: Style = {
	let mut style = Style::DEFAULT;
	style.width = Val::Px(256.0);
	style.height = Val::Px(256.0);
	style.margin = UiRect {
		left: Val::Px(8.0),
		right: Val::Px(8.0),
		top: Val::Px(8.0),
		bottom: Val::Px(8.0),
	};
	style
};

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
