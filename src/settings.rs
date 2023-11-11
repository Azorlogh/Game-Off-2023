use std::{fs::read_to_string, collections::HashMap, path::PathBuf, ops::AddAssign};

use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{input::Inputs, menu::GeneralInput};
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
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
            Movement::Eat => String::from("Eat"),
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

            Movement::Eat => {},

            Movement::Void => {}
        };
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Movement::Right,
            Movement::Left,
            Movement::Forward,
            Movement::Backward,
            Movement::Jump,
            Movement::Punch,
            Movement::Yaw(Some(true)),
            Movement::Yaw(Some(false)),
            Movement::Pitch(Some(true)),
            Movement::Pitch(Some(false)),
            Movement::Eat,
            Movement::Void
        ].iter().copied()
    }
}
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord)]
pub struct Motion(pub usize);

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub input: HashMap<GeneralInput, Movement>
}

impl Settings {
    pub fn length_motion(&self) -> usize {
        self.input.keys().filter(|k| matches!(k, GeneralInput::Motion(_))).count()
    }
}

impl AddAssign for Settings {
    fn add_assign(&mut self, rhs: Self) {
        self.input.extend(rhs.input);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            input: HashMap::from([
                (GeneralInput::KeyCode(KeyCode::Z), Movement::Forward),
                (GeneralInput::KeyCode(KeyCode::S), Movement::Backward),
                (GeneralInput::KeyCode(KeyCode::Q), Movement::Left),
                (GeneralInput::KeyCode(KeyCode::D), Movement::Right),
                (GeneralInput::KeyCode(KeyCode::Space), Movement::Jump),
                (GeneralInput::KeyCode(KeyCode::T), Movement::Yaw(Some(true))),
                (GeneralInput::KeyCode(KeyCode::B), Movement::Yaw(Some(false))),
                (GeneralInput::KeyCode(KeyCode::E), Movement::Eat),

                (GeneralInput::MouseButton(MouseButton::Left), Movement::Punch),

                (GeneralInput::Motion(0), Movement::Yaw(None)),
                (GeneralInput::Motion(1), Movement::Pitch(None)),
            ])
        }
    }
}