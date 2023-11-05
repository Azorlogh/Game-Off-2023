use bevy::{
	input::{mouse::MouseMotion, InputSystem},
	prelude::*,
	window::{CursorGrabMode, PrimaryWindow},
};

const DEADZONE: f32 = 0.2;

pub struct InputPlugin;
impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Inputs>()
			.add_systems(Update, capture_mouse)
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
					.after(InputSystem),
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
				gamepad: gamepad,
				axis_type: GamepadAxisType::LeftStickX,
			})
			.unwrap();
		deadzone_symmetric(val)
	};

	inputs.dir.y = {
		let val = gamepad_axes
			.get(GamepadAxis {
				gamepad: gamepad,
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

fn handle_keyboard_input(mut inputs: ResMut<Inputs>, keys: Res<Input<KeyCode>>) {
	if keys.pressed(KeyCode::W) {
		inputs.dir.y += 1.0;
	}
	if keys.pressed(KeyCode::S) {
		inputs.dir.y += -1.0;
	}
	if keys.pressed(KeyCode::A) {
		inputs.dir.x += -1.0;
	}
	if keys.pressed(KeyCode::D) {
		inputs.dir.x += 1.0;
	}
	if keys.pressed(KeyCode::Space) {
		inputs.jump = true;
	}
}

fn handle_mouse_input(
	time: Res<Time>,
	mut inputs: ResMut<Inputs>,
	buttons: Res<Input<MouseButton>>,
	mut mouse_motion: EventReader<MouseMotion>,
) {
	let delta = mouse_motion.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);
	inputs.pitch += delta.y / (time.delta_seconds().max(0.001)) * -1e-5;
	inputs.yaw += delta.x / (time.delta_seconds().max(0.001)) * -1e-5;
	inputs.punch |= buttons.pressed(MouseButton::Right);
}

fn finalize_input(mut inputs: ResMut<Inputs>) {
	if inputs.dir.length() > 1.0 {
		inputs.dir = inputs.dir.normalize();
	}
}
