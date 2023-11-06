use std::{fs::read_to_string, collections::HashMap};

use bevy::{prelude::*, input::mouse::MouseMotion};

use serde::{Deserialize, Serialize};

use crate::input::Inputs;
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
	fn build(&self, app: &mut App) {
        // Load Settings
		let settings = match read_to_string("assets/settings.ron") {
            Ok(s) => match ron::from_str::<Settings>(&s) {
                Ok(s) => s,
                Err(e) => {
                    warn!("failed to load settings, using defaults: {e}");
                    Settings::default()
                },
            },
            Err(e) => {
                warn!("failed to load settings, using defaults: {e}");
                Settings::default()
            }
        };
        
		app.insert_resource(settings);
	}
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Movement {
    Right,
    Left,
    Forward,
    Backward,
    Jump,
    Punch,
    Yaw(Option<bool>),
    Pitch(Option<bool>)
}

impl Movement {
    pub fn input(&self, inputs: &mut Inputs, modifier: Vec2) {
        match self {
            Movement::Right => { inputs.dir.x += 1.0 } ,
            Movement::Left => { inputs.dir.x -= 1.0 },
            Movement::Forward => { inputs.dir.y += 1.0 },
            Movement::Backward => { inputs.dir.y -= 1.0 },
            Movement::Jump => { inputs.jump = true },
            Movement::Punch => { inputs.punch = true },
            Movement::Yaw(Some(t)) => { inputs.yaw += 0.1 * modifier.x * if *t {-1.0} else {1.0} },
            Movement::Pitch(Some(t)) => { inputs.pitch += 0.1 * modifier.x * if *t {-1.0} else {1.0} },

            Movement::Yaw(None) => { inputs.yaw += modifier.x }
            Movement::Pitch(None) => { inputs.pitch += modifier.y }
            _ => {}
        };
    }
}
#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub keyboard_input: HashMap<KeyCode, Movement>,
    pub mouse_input: HashMap<MouseButton, Movement>,
    pub mouse_motion: Option<Vec<Movement>>
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            keyboard_input: HashMap::from([
                (KeyCode::Z, Movement::Forward),
                (KeyCode::S, Movement::Backward),
                (KeyCode::Q, Movement::Left),
                (KeyCode::D, Movement::Right),
                (KeyCode::Space, Movement::Jump),
                (KeyCode::T, Movement::Yaw(Some(true))),
                (KeyCode::B, Movement::Yaw(Some(false))),
            ]),
            mouse_input: HashMap::from([
                (MouseButton::Right, Movement::Punch),
                (MouseButton::Left, Movement::Jump)

            ]),
            mouse_motion: Some(vec![Movement::Yaw(None), Movement::Pitch(None)])
        }
    }
}