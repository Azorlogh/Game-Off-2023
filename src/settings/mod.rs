use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::input::Inputs;

pub struct SettingsPlugin;

use fs::load_settings;
pub(crate) mod fs;

impl Plugin for SettingsPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(load_settings())
			.add_systems(Update, save_settings);
	}
}

fn save_settings(settings: Res<Settings>) {
	if settings.is_changed() {
		fs::save_settings(&settings);
	}
}

#[derive(Resource, Eq, Hash, PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GeneralInput {
	KeyCode(KeyCode),
	MouseButton(MouseButton),
	Motion,
}

impl std::fmt::Display for GeneralInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			GeneralInput::KeyCode(x) => write!(f, "{:?}", x),
			GeneralInput::MouseButton(x) => write!(f, "{:?}", x),
			GeneralInput::Motion => write!(f, ""),
		}
	}
}

#[derive(
	Debug, Serialize, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy, Resource,
)]
pub enum Action {
	Right,
	Left,
	Forward,
	Backward,
	Jump,
	Punch,
	Yaw(Option<bool>),
	Pitch(Option<bool>),
	Eat,
}

impl ToString for Action {
	fn to_string(&self) -> String {
		match self {
			Action::Right => String::from("Right"),
			Action::Left => String::from("Left"),
			Action::Forward => String::from("Forward"),
			Action::Backward => String::from("Backward"),
			Action::Jump => String::from("Jump"),
			Action::Punch => String::from("Punch"),
			Action::Yaw(o) => match o {
				Some(b) => format!("X Vision Keyboard {}", if *b { "-" } else { "+" }),
				None => String::from("X Vision Mouse"),
			},
			Action::Pitch(o) => match o {
				Some(b) => format!("Y Vision Keyboard {}", if *b { "-" } else { "+" }),
				None => String::from("Y Vision Mouse"),
			},
			Action::Eat => String::from("Eat"),
		}
	}
}

impl Action {
	pub fn input(&self, inputs: &mut Inputs, modifier: Vec2) {
		match self {
			Action::Right => inputs.dir.x += 1.0,
			Action::Left => inputs.dir.x -= 1.0,
			Action::Forward => inputs.dir.y += 1.0,
			Action::Backward => inputs.dir.y -= 1.0,
			Action::Jump => inputs.jump = true,
			Action::Punch => inputs.punch = true,
			Action::Yaw(Some(t)) => inputs.yaw += 0.1 * modifier.x * if *t { -1.0 } else { 1.0 },
			Action::Pitch(Some(t)) => {
				inputs.pitch += 0.1 * modifier.x * if *t { -1.0 } else { 1.0 }
			}

			Action::Yaw(None) => inputs.yaw += modifier.x,
			Action::Pitch(None) => inputs.pitch += modifier.y,

			Action::Eat => inputs.eat = true,
		};
	}
}

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
	pub input: HashMap<Action, GeneralInput>,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			input: HashMap::from([
				(Action::Right, GeneralInput::KeyCode(KeyCode::D)),
				(Action::Left, GeneralInput::KeyCode(KeyCode::A)),
				(Action::Forward, GeneralInput::KeyCode(KeyCode::W)),
				(Action::Backward, GeneralInput::KeyCode(KeyCode::S)),
				(Action::Jump, GeneralInput::KeyCode(KeyCode::Space)),
				(Action::Punch, GeneralInput::MouseButton(MouseButton::Left)),
				(Action::Yaw(None), GeneralInput::Motion),
				// (Action::Yaw(Some(false)), GeneralInput::KeyCode(KeyCode::T)),
				// (Action::Yaw(Some(true)), GeneralInput::KeyCode(KeyCode::G)),
				(Action::Pitch(None), GeneralInput::Motion),
				// (
				// 	Action::Pitch(Some(false)),
				// 	GeneralInput::KeyCode(KeyCode::R),
				// ),
				// (Action::Pitch(Some(true)), GeneralInput::KeyCode(KeyCode::F)),
				(Action::Eat, GeneralInput::KeyCode(KeyCode::E)),
			]),
		}
	}
}
