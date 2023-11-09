use bevy::{
	input::{mouse::MouseMotion, InputSystem, Input},
	prelude::*,
	window::{CursorGrabMode, PrimaryWindow},
};
use std::hash::Hash;

use crate::{settings::{Settings, Motion}, GameState, menu::{MenuState, OptionState, GetInput, GetType}};

const DEADZONE: f32 = 0.2;

pub struct InputPlugin;
impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Inputs>()
			.add_systems(Update, capture_mouse.run_if(in_state(GameState::Running)))
			.add_systems(PreUpdate, handle_menu.run_if(in_state(GameState::Running).or_else(in_state(GameState::Pause))))
			.add_systems(Update,
				(
					get_input_to_settings_input::<KeyCode>,
					get_input_to_settings_input::<MouseButton>,
					get_input_to_settings_motion,
				).run_if(in_state(MenuState::Option).and_then(in_state(OptionState::WaitInput))))
			.add_systems(
				PreUpdate,
				(
					reset_input,
					handle_gamepad_input,
					handle_keyboard_input,
					handle_mouse_input,
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
		warn!("gamepad not connected");
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

fn handle_keyboard_input(mut inputs: ResMut<Inputs>, keys: Res<Input<KeyCode>>, settings: Res<Settings>, time: Res<Time>) {
	for key in keys.get_pressed() {
		match settings.keyboard_input.get(key) {
			Some(i) => i.input(&mut inputs, Vec2::new(time.delta_seconds() * 35.0, 0.0)),
			None => {},
		};
	}
}

fn handle_menu(keys: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<GameState>>, state: Res<State<GameState>>, menu_state: Res<State<MenuState>>) {
	if keys.just_pressed(KeyCode::Escape) && menu_state.get() == &MenuState::Menu {
		match state.get() {
			GameState::Running => app_state.set(GameState::Pause),
			GameState::Pause => app_state.set(GameState::Running),
    		_ => {},
		};
	}
}

fn handle_mouse_input(
	time: Res<Time>,
	mut inputs: ResMut<Inputs>,
	buttons: Res<Input<MouseButton>>,
	mut mouse_motion: EventReader<MouseMotion>,
	settings: Res<Settings>
) {
	let delta = mouse_motion.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);
	for (_, mov) in &settings.mouse_motion {
		mov.input(&mut inputs, delta / (time.delta_seconds().max(0.001)) * -1e-5);
	}

	for button in buttons.get_pressed() {
		match settings.mouse_input.get(button) {
			Some(i) => i.input(&mut inputs, Vec2::new(time.delta_seconds() * 35.0, 0.0)),
			None => {},
		};
	}
}

fn finalize_input(mut inputs: ResMut<Inputs>) {
	if inputs.dir.length() > 1.0 {
		inputs.dir = inputs.dir.normalize();
	}
}



fn get_input_to_settings_input<T> (
	p: Res<Input<T>>,
	mut option_state: ResMut<NextState<OptionState>>,
	mut command: Commands
) where T: GetType + Copy + Eq + Hash + Send + Sync {
	for k in p.get_just_pressed() {
		command.insert_resource(GetInput(*k));
		option_state.set(OptionState::AddInput);
	}
}

fn get_input_to_settings_motion(
	mut motion: EventReader<MouseMotion>,
	mut option_state: ResMut<NextState<OptionState>>,
	mut command: Commands,
	settings: Res<Settings>
) {
	let delta = motion.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);
	if delta.length() > 0.0 {
		command.insert_resource(GetInput(Motion(settings.length_motion())));
		option_state.set(OptionState::AddInput);
	}
}
