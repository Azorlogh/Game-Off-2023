use std::{fs::read_to_string, collections::HashMap, path::PathBuf, ops::AddAssign};

use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use crate::input::Inputs;
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(load_settings());
	}
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    // Load Settings
    match read_to_string(path) {
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
    }
}

fn settings_path() -> PathBuf {
    directories::ProjectDirs::from("", "NeuroControls", "GameOff")
    .unwrap()
    .config_dir()
    .join("settings.ron")
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Movement {
    Right,
    Left,
    Forward,
    Backward,
    Jump,
    Punch,
    Yaw(Option<bool>),
    Pitch(Option<bool>),
    Void
}

impl ToString for Movement {
    fn to_string(&self) -> String {
        match self {
            Movement::Right => String::from("Right"),
            Movement::Left => String::from("Left"),
            Movement::Forward => String::from("Forward"),
            Movement::Backward => String::from("Backward"),
            Movement::Jump => String::from("Jump"),
            Movement::Punch => String::from("Punch"),
            Movement::Yaw(_) => String::from("X Vision"),
            Movement::Pitch(_) => String::from("Y Vision"),
            Movement::Void => String::from("")
        }
    }
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

            Movement::Yaw(None) => { inputs.yaw += modifier.x },
            Movement::Pitch(None) => { inputs.pitch += modifier.y },

            Movement::Void => {}
        };
    }
}
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord)]
pub struct Motion(pub usize);

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub keyboard_input: HashMap<KeyCode, Movement>,
    pub mouse_input: HashMap<MouseButton, Movement>,
    pub mouse_motion: HashMap<Motion, Movement>
}

impl Settings {
    pub fn is_void(self) -> Option<Self> {
        if self.keyboard_input.len() > 0 || self.mouse_input.len() > 0 || self.mouse_motion.len() > 0 {
            return Some(self);
        }
        None
    }
    pub fn length_motion(&self) -> usize {
        self.mouse_motion.len()
    }
}

impl AddAssign for Settings {
    fn add_assign(&mut self, rhs: Self) {
        self.keyboard_input.extend(rhs.keyboard_input);
        self.mouse_input.extend(rhs.mouse_input);
        self.mouse_motion.extend(rhs.mouse_motion);
    }
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
            mouse_motion: HashMap::from([
                (Motion(0), Movement::Yaw(None)),
                (Motion(1), Movement::Pitch(None)
            )])
        }
    }
}
