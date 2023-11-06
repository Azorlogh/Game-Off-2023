use std::{fs::read_to_string, collections::HashMap};

use bevy::prelude::*;

use serde::{Deserialize, Serialize};
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
	fn build(&self, app: &mut App) {
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
pub enum Input {
    Right,
    Left,
    Forward,
    Backward,
    Jump
}
#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub keyboard_input: HashMap<Input, KeyCode>,
}
impl Default for Settings {
    fn default() -> Self {
        Self { keyboard_input: HashMap::from([
            (Input::Forward, KeyCode::W),
            (Input::Left, KeyCode::A),
            (Input::Right, KeyCode::D),
            (Input::Backward, KeyCode::S),
            (Input::Jump, KeyCode::Space),
        ])}
    }
}