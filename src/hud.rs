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

pub struct HudPlugin;
impl Plugin for HudPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup)
			.add_systems(Update, player_status_ui);
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
}

fn player_status_ui(
	mut painter: ShapePainter,
	q_player: Query<(&Health, &Hydration, &Glucose), With<Player>>,
) {
	painter.render_layers = Some(RenderLayers::layer(1));
	let Ok((health, hydration, glucose)) = q_player.get_single() else {
		return;
	};

	show_bar(&mut painter, health.current / health.max, -0.7, Color::RED);
	show_bar(&mut painter, hydration.0, -0.8, Color::CYAN);
	show_bar(&mut painter, glucose.0, -0.9, Color::YELLOW);
}

fn show_bar(painter: &mut ShapePainter, portion: f32, y_offset: f32, color: Color) {
	const BAR_SIZE: f32 = 2.0;

	let start_x = -BAR_SIZE / 2.0;
	let end_x = BAR_SIZE / 2.0;
	painter.thickness = 0.02;
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
}
