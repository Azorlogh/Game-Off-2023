use std::{fs::read_to_string, collections::HashMap, path::PathBuf};

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

pub fn save_settings(settings: &Settings) {
    let path = settings_path();
    // Save Settings
    match ron::to_string(settings) {
        Ok(s) => match std::fs::write(path.clone(), s.clone()) {
            Ok(_) => {},
            Err(_) => {
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, s).unwrap();
            }
        },
        Err(e) => warn!("failed to save settings: {e}")
    }
}

fn settings_path() -> PathBuf {
    directories::ProjectDirs::from("", "NeuroControls", "GameOff")
    .unwrap()
    .config_dir()
    .join("settings.ron")
}


#[derive(Resource, Eq, Hash, PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GeneralInput {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
    Motion
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy, Resource)]
pub enum Movement {
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

impl ToString for Movement {
    fn to_string(&self) -> String {
        match self {
            Movement::Right => String::from("Right"),
            Movement::Left => String::from("Left"),
            Movement::Forward => String::from("Forward"),
            Movement::Backward => String::from("Backward"),
            Movement::Jump => String::from("Jump"),
            Movement::Punch => String::from("Punch"),
            Movement::Yaw(o) => match o {
                Some(b) => format!("X Vision Keyboard {}", if *b {"-"} else {"+"}),
                None => String::from("X Vision Mouse"),
            },
            Movement::Pitch(o) => match o {
                Some(b) => format!("Y Vision Keyboard {}", if *b {"-"} else {"+"}),
                None => String::from("Y Vision Mouse"),
            },
            Movement::Eat => String::from("Eat"),
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

            Movement::Eat => {},
        };
    }
}
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord)]
pub struct Motion(pub usize);

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub input: HashMap<Movement, GeneralInput>
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            input: HashMap::from([
                (Movement::Right, GeneralInput::KeyCode(KeyCode::D)),
                (Movement::Left, GeneralInput::KeyCode(KeyCode::A)),
                (Movement::Forward, GeneralInput::KeyCode(KeyCode::W)),
                (Movement::Backward, GeneralInput::KeyCode(KeyCode::S)),
                (Movement::Jump, GeneralInput::KeyCode(KeyCode::Space)),
                (Movement::Punch, GeneralInput::MouseButton(MouseButton::Left)),
                (Movement::Yaw(None), GeneralInput::Motion),
                (Movement::Yaw(Some(false)), GeneralInput::KeyCode(KeyCode::T)),
                (Movement::Yaw(Some(true)), GeneralInput::KeyCode(KeyCode::G)),
                (Movement::Pitch(None), GeneralInput::Motion),
                (Movement::Pitch(Some(false)), GeneralInput::KeyCode(KeyCode::R)),
                (Movement::Pitch(Some(true)), GeneralInput::KeyCode(KeyCode::F)),
                (Movement::Eat, GeneralInput::MouseButton(MouseButton::Right)),
            ])
        }
    }
}
