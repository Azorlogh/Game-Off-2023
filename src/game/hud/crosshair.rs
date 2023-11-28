use std::f32::consts::TAU;

use bevy::{prelude::*, render::view::RenderLayers};
use bevy_vector_shapes::{
	painter::ShapePainter,
	shapes::{DiscPainter, LinePainter},
};

use crate::game::{food::components::FoodProperties, player::eat::EatingState};

pub struct CrosshairPlugin;
impl Plugin for CrosshairPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (croshair, eating_indicator));
	}
}

const SIZE: f32 = 0.02;

fn croshair(mut painter: ShapePainter) {
	painter.set_2d();
	painter.render_layers = Some(RenderLayers::layer(1));

	painter.color = Color::WHITE;
	painter.thickness = 0.002;
	painter.line(Vec3::new(-SIZE, 0.0, 0.0), Vec3::new(SIZE, 0.0, 0.0));
	painter.line(Vec3::new(0.0, -SIZE, 0.0), Vec3::new(0.0, SIZE, 0.0));
}

fn eating_indicator(
	mut painter: ShapePainter,
	q_food: Query<&FoodProperties>,
	eating_state: Res<EatingState>,
) {
	match *eating_state {
		EatingState::Eating(food_entity, time) => {
			painter.set_2d();
			painter.render_layers = Some(RenderLayers::layer(1));
			painter.thickness = 0.006;
			painter.hollow = true;
			painter.color = Color::ORANGE_RED;
			let time_per_bite = q_food.get(food_entity).unwrap().time_per_bite;
			let portion = time / time_per_bite;
			painter.arc(SIZE * 2.0, -TAU / 3.0, (-1.0 + portion * 2.0) * TAU / 3.0);
		}
		EatingState::Idle => {}
	}
}
