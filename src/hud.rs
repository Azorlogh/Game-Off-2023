use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	prelude::*,
	render::{camera::ScalingMode, view::RenderLayers},
};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::LinePainter};

use crate::{
	health::Health,
	player::{
		nutrition::{Glucose, Hydration},
		Player,
	},
};

const BAR_LENGTH: f32 = 1.0;
const BAR_WIDTH: f32 = 0.03;
const TEXT_SCALE: f32 = BAR_WIDTH / 20.0;
const FONT_SIZE: f32 = 36.0;
const HEALTH_OFFSET: f32 = -0.7;
const HYDRATION_OFFSET: f32 = -0.75;
const GLUCOSE_OFFSET: f32 = -0.8;
const RIGHT_LABEL_OFFSET: f32 = BAR_LENGTH / 2.0 + 0.1;
const LEFT_LABEL_OFFSET: f32 = -BAR_LENGTH / 2.0 - 0.15;

pub struct HudPlugin;
impl Plugin for HudPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup)
			.add_systems(Update, player_status_ui);
	}
}

#[derive(Component)]
pub struct HealthValue;

#[derive(Component)]
pub struct GlucoseValue;

#[derive(Component)]
pub struct HydrationValue;

#[derive(Component)]
pub struct HealthLabel;

#[derive(Component)]
pub struct GlucoseLabel;

#[derive(Component)]
pub struct HydrationLabel;

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
		Camera2dBundle {
			camera: Camera {
				order: 1,
				..default()
			},
			camera_2d: Camera2d {
				clear_color: ClearColorConfig::None,
			},
			projection: OrthographicProjection {
				scaling_mode: ScalingMode::FixedVertical(2.0),
				..default()
			},
			..default()
		},
		RenderLayers::layer(1),
	));

	commands.spawn((
		create_label(
			String::from("100"),
			Vec3::new(RIGHT_LABEL_OFFSET, HEALTH_OFFSET, 0.0),
		),
		HealthValue,
		RenderLayers::layer(1),
	));

	commands.spawn((
		create_label(
			String::from(""),
			Vec3::new(RIGHT_LABEL_OFFSET, HYDRATION_OFFSET, 0.0),
		),
		HydrationValue,
		RenderLayers::layer(1),
	));

	commands.spawn((
		create_label(
			String::from(""),
			Vec3::new(RIGHT_LABEL_OFFSET, GLUCOSE_OFFSET, 0.0),
		),
		GlucoseValue,
		RenderLayers::layer(1),
	));

	commands.spawn((
		create_label(
			String::from("HEALTH"),
			Vec3::new(LEFT_LABEL_OFFSET, HEALTH_OFFSET, 0.0),
		),
		HealthLabel,
		RenderLayers::layer(1),
	));

	commands.spawn((
		create_label(
			String::from("HYDRATION"),
			Vec3::new(LEFT_LABEL_OFFSET, HYDRATION_OFFSET, 0.0),
		),
		HydrationLabel,
		RenderLayers::layer(1),
	));

	commands.spawn((
		create_label(
			String::from("GLUCOSE"),
			Vec3::new(LEFT_LABEL_OFFSET, GLUCOSE_OFFSET, 0.0),
		),
		GlucoseLabel,
		RenderLayers::layer(1),
	));
}

fn player_status_ui(
	mut painter: ShapePainter,
	q_player: Query<(&Health, &Hydration, &Glucose), With<Player>>,
	mut q_health: Query<&mut Text, With<HealthValue>>,
	mut q_hydration: Query<&mut Text, (With<HydrationValue>, Without<HealthValue>)>,
	mut q_glucose: Query<
		&mut Text,
		(
			With<GlucoseValue>,
			Without<HealthValue>,
			Without<HydrationValue>,
		),
	>,
) {
	painter.render_layers = Some(RenderLayers::layer(1));
	let Ok((health, hydration, glucose)) = q_player.get_single() else {
		return;
	};

	// bar
	show_bar(
		&mut painter,
		(health.current / health.max).min(1.0),
		HEALTH_OFFSET,
		Color::RED,
		&mut q_health.single_mut().sections[0].value,
	);
	show_bar(
		&mut painter,
		(hydration.0).min(1.0),
		HYDRATION_OFFSET,
		Color::CYAN,
		&mut q_hydration.single_mut().sections[0].value,
	);
	show_bar(
		&mut painter,
		(glucose.0).min(1.0),
		GLUCOSE_OFFSET,
		Color::YELLOW,
		&mut q_glucose.single_mut().sections[0].value,
	);
}

fn show_bar(
	painter: &mut ShapePainter,
	portion: f32,
	y_offset: f32,
	color: Color,
	label: &mut String,
) {
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

	*label = portion.to_string();
}
