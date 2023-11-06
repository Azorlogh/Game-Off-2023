use std::{fs::read_to_string, path::PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};


pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
	fn build(&self, app: &mut App) {
		// let settings = load_settings();
		// app.insert_resource(settings.nickname)
		// 	.insert_resource(settings.input_mapping)
		// 	.add_event::<SaveSettings>()
		// 	.add_system(settings_save);
	}
}
