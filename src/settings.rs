use std::{fs::read_to_string, collections::HashMap};

use bevy::prelude::*;

use serde::{Deserialize, Serialize};
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
    Punch
}
#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub keyboard_input: HashMap<Movement, KeyCode>,
    pub mouse_input: HashMap<Movement, MouseButton>
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            keyboard_input: HashMap::from([
                (Movement::Forward, KeyCode::W),
                (Movement::Left, KeyCode::A),
                (Movement::Right, KeyCode::D),
                (Movement::Backward, KeyCode::S),
                (Movement::Jump, KeyCode::Space),
            ]),
            mouse_input: HashMap::from([
                (Movement::Punch, MouseButton::Right)
            ])
        }
    }
}