use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::settings::*;

use super::OptionState;

pub(super) fn transfer_input (
    input: Res<GeneralInput>,
    mut option_state: ResMut<NextState<OptionState>>,
    mut settings: ResMut<Settings>,
    mut command: Commands
) {
    if let Some(s) = input.to_settings().is_void() {
        *settings += s;
        command.remove_resource::<GeneralInput>();
        option_state.set(OptionState::Option);
    }
}

#[derive(Resource, Eq, Hash, PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GeneralInput {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
    Motion(usize)
}


impl GeneralInput {
    fn to_settings(&self) -> Settings {
        Settings { input: HashMap::from([(*self, Movement::Void)]) }
    }
}
