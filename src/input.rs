use std::hash::Hash;

use bevy::{
	input::{mouse::MouseMotion, Input, InputSystem},
	prelude::*,
	window::{CursorGrabMode, PrimaryWindow},
};

use crate::{settings::{Settings, GeneralInput}, GameState, menu::MenuState};

const DEADZONE: f32 = 0.2;

pub struct InputPlugin;
impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Inputs>()
			.add_systems(Update, capture_mouse.run_if(in_state(GameState::Running)))
			.add_systems(PreUpdate, handle_menu.run_if(in_state(GameState::Running).or_else(in_state(GameState::Pause))))
			.add_systems(
				PreUpdate,
				(
					reset_input,
					handle_gamepad_input,
					handle_inputs,
					finalize_input,
				)
					.chain()
					.in_set(InputSet)
					.after(InputSystem)
					.run_if(in_state(GameState::Running)),
			);
	}
}

fn capture_mouse(
	mut q_window: Query<&mut Window, With<PrimaryWindow>>,
	buttons: Res<Input<MouseButton>>,
	keys: Res<Input<KeyCode>>,
) {
	let mut window = q_window.single_mut();
	match window.cursor.grab_mode {
		CursorGrabMode::None if buttons.just_pressed(MouseButton::Left) => {
			window.cursor.grab_mode = CursorGrabMode::Locked;
			window.cursor.visible = false;
		}
		_ if keys.just_pressed(KeyCode::Escape) => {
			window.cursor.grab_mode = CursorGrabMode::None;
			window.cursor.visible = true;
		}
		_ => {}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct InputSet;

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct Inputs {
	pub dir: Vec2,
	pub pitch: f32,
	pub yaw: f32,
	pub punch: bool,
	pub jump: bool,
}

impl Default for Inputs {
	fn default() -> Self {
		Self {
			dir: Vec2::ZERO,
			pitch: 0.0,
			yaw: 0.0,
			punch: false,
			jump: false,
		}
	}
}

fn reset_input(mut inputs: ResMut<Inputs>) {
	*inputs = Inputs::default();
}

fn handle_gamepad_input(
	mut inputs: ResMut<Inputs>,
	gamepads: Res<Gamepads>,
	gamepad_axes: Res<Axis<GamepadAxis>>,
	_gamepad_axes2: Res<Axis<GamepadButton>>,
	gamepad_buttons: Res<Input<GamepadButton>>,
) {
	let Some(gamepad) = gamepads.iter().next() else {
		return;
	};

	fn deadzone_symmetric(x: f32) -> f32 {
		((x.abs() - DEADZONE) / (1.0 - DEADZONE)).max(0.0) * x.signum()
	}

	inputs.dir.x = {
		let val = gamepad_axes
			.get(GamepadAxis {
				gamepad,
				axis_type: GamepadAxisType::LeftStickX,
			})
			.unwrap();
		deadzone_symmetric(val)
	};

	inputs.dir.y = {
		let val = gamepad_axes
			.get(GamepadAxis {
				gamepad,
				axis_type: GamepadAxisType::LeftStickY,
			})
			.unwrap();
		deadzone_symmetric(val)
	};

	inputs.yaw = {
		let val = gamepad_axes
			.get(GamepadAxis {
				gamepad,
				axis_type: GamepadAxisType::RightStickX,
			})
			.unwrap();
		deadzone_symmetric(val)
	};

	inputs.pitch = {
		let val = gamepad_axes
			.get(GamepadAxis {
				gamepad,
				axis_type: GamepadAxisType::RightStickY,
			})
			.unwrap();
		deadzone_symmetric(val)
	};

	inputs.jump = gamepad_buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::South));

	inputs.punch = gamepad_buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::East));
}

fn handle_inputs(
	mut inputs: ResMut<Inputs>,
	buttons: Res<Input<MouseButton>>,
	keys: Res<Input<KeyCode>>,
	mut mouse_motion: EventReader<MouseMotion>,
	settings: Res<Settings>,
	time: Res<Time>,
) {
	let delta = mouse_motion.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);

	for (m, k) in settings.input.iter() {
		match k {
			GeneralInput::KeyCode(key) => {
				if keys.pressed(*key) {
					m.input(&mut inputs, Vec2::new(time.delta_seconds() * 35.0, 0.0));
				}
			}
			GeneralInput::MouseButton(button) => {
				if buttons.pressed(*button) {
					m.input(&mut inputs, Vec2::new(time.delta_seconds() * 35.0, 0.0));
				}
			},
			GeneralInput::Motion => {
				m.input(&mut inputs, delta / (time.delta_seconds().max(0.001)) * -1e-5);
			},
		};
	}
}

fn finalize_input(mut inputs: ResMut<Inputs>) {
	if inputs.dir.length() > 1.0 {
		inputs.dir = inputs.dir.normalize();
	}
}

fn handle_menu(
	keys: Res<Input<KeyCode>>,
	mut app_state: ResMut<NextState<GameState>>,
	state: Res<State<GameState>>,
	menu_state: Res<State<MenuState>>,
) {
	if keys.just_pressed(KeyCode::Escape) && menu_state.get() == &MenuState::Menu {
		match state.get() {
			GameState::Running => app_state.set(GameState::Pause),
			GameState::Pause => app_state.set(GameState::Running),
			_ => {}
		};
	}
}
