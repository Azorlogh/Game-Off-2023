use bevy::{prelude::*, render::view::RenderLayers};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::LinePainter};

use crate::{
	game::{
		health::Health,
		player::{calories::Calories, Player},
		DespawnOnExitGame,
	},
	AppState,
};

const BAR_LENGTH: f32 = 1.0;
const BAR_WIDTH: f32 = 0.03;
const TEXT_SCALE: f32 = BAR_WIDTH / 20.0;
const FONT_SIZE: f32 = 36.0;
const HEALTH_OFFSET: f32 = -0.7;
const CALORIES_OFFSET: f32 = -0.75;
const RIGHT_LABEL_OFFSET: f32 = BAR_LENGTH / 2.0 + 0.1;
const LEFT_LABEL_OFFSET: f32 = -BAR_LENGTH / 2.0 - 0.15;

pub struct PlayerUiPlugin;
impl Plugin for PlayerUiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::Game), setup)
			.add_systems(Update, player_status_ui);
	}
}

#[derive(Component)]
pub struct HealthValue;

#[derive(Component)]
pub struct GlucoseValue;

#[derive(Component)]
pub struct CaloriesValue;

#[derive(Component)]
pub struct HealthLabel;

#[derive(Component)]
pub struct GlucoseLabel;

#[derive(Component)]
pub struct CaloriesLabel;

pub fn create_label(label: String, position: Vec3) -> Text2dBundle {
	Text2dBundle {
		text: Text::from_section(
			label,
			TextStyle {
				font_size: FONT_SIZE,
				..default()
			},
		),
		transform: Transform::from_translation(position).with_scale(Vec3::splat(TEXT_SCALE)),
		..default()
	}
}

fn setup(mut commands: Commands) {
	commands.spawn((
		create_label(
			String::from("100"),
			Vec3::new(RIGHT_LABEL_OFFSET, HEALTH_OFFSET, 0.0),
		),
		HealthValue,
		RenderLayers::layer(1),
		DespawnOnExitGame,
	));

	commands.spawn((
		create_label(
			String::from(""),
			Vec3::new(RIGHT_LABEL_OFFSET, CALORIES_OFFSET, 0.0),
		),
		CaloriesValue,
		RenderLayers::layer(1),
		DespawnOnExitGame,
	));

	commands.spawn((
		create_label(
			String::from("HEALTH"),
			Vec3::new(LEFT_LABEL_OFFSET, HEALTH_OFFSET, 0.0),
		),
		HealthLabel,
		RenderLayers::layer(1),
		DespawnOnExitGame,
	));

	commands.spawn((
		create_label(
			String::from("CALORIES"),
			Vec3::new(LEFT_LABEL_OFFSET, CALORIES_OFFSET, 0.0),
		),
		CaloriesLabel,
		RenderLayers::layer(1),
		DespawnOnExitGame,
	));
}

fn player_status_ui(
	mut painter: ShapePainter,
	q_player: Query<(&Health, &Calories), With<Player>>,
	mut q_health: Query<&mut Text, With<HealthValue>>,
	mut q_calories: Query<&mut Text, (With<CaloriesValue>, Without<HealthValue>)>,
) {
	painter.set_2d();
	painter.render_layers = Some(RenderLayers::layer(1));
	let Ok((health, calories)) = q_player.get_single() else {
		return;
	};

	// bar
	show_bar(
		&mut painter,
		(health.current as f32 / health.max as f32).min(1.0),
		HEALTH_OFFSET,
		Color::RED,
		&mut q_health.single_mut().sections[0].value,
	);
	show_bar(
		&mut painter,
		(calories.0).min(100.0) as f32 / 100.0,
		CALORIES_OFFSET,
		Color::CYAN,
		&mut q_calories.single_mut().sections[0].value,
	);
}

fn show_bar(
	painter: &mut ShapePainter,
	portion: f32,
	y_offset: f32,
	color: Color,
	label: &mut String,
) {
	let portion = portion.clamp(0.0, 1.0);
	let start_x = -BAR_LENGTH / 2.0;
	let end_x = BAR_LENGTH / 2.0;
	painter.thickness = BAR_WIDTH;
	painter.color = Color::BLACK;
	painter.line(
		Vec3::new(start_x, y_offset, 0.0),
		Vec3::new(end_x, y_offset, 0.0),
	);
	painter.color = color;
	painter.line(
		Vec3::new(start_x, y_offset, 0.0),
		Vec3::new(start_x * (1.0 - portion) + end_x * portion, y_offset, 0.0),
	);

	*label = ((portion * 100.0).ceil() as i32).to_string();
}
