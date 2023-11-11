use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::settings::*;

use super::{OptionState, LastInput};

pub(super) fn transfer_input (
    input: Res<GeneralInput>,
    mut option_state: ResMut<NextState<OptionState>>,
    mut settings: ResMut<Settings>,
    last_input: Option<Res<LastInput>>,
    mut command: Commands
) {
    if let Some(last_input) = last_input {
        if let Some((_, last_value)) = settings.input.remove_entry(&last_input.0) {
            settings.input.insert(input.clone(), last_value);
            command.remove_resource::<LastInput>();
        }
    }
    else {
        *settings += input.to_settings();
    }
    command.remove_resource::<GeneralInput>();
    option_state.set(OptionState::Option);
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
