use std::collections::HashMap;
use std::hash::Hash;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use ron::Error;
use serde::Serialize;
use super::MenuState;

use crate::settings::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum OptionState {
    #[default]
    Option,
    WaitInput,
    AddInput
}

pub(super) fn ui_options(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut option_state: ResMut<NextState<OptionState>>,
    settings: Res<Settings>
) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Back").clicked() {
            menu_state.set(MenuState::Menu);
        }
        for (k, m) in settings.keyboard_input.iter() {
            if ui.button(format!("{:?}", k)).clicked() {
                option_state.set(OptionState::WaitInput);
                // wait input
            }

            if ui.button(m.to_string()).clicked() {
                // set List Selector
            }
        }

        if ui.button("+").clicked() {
            option_state.set(OptionState::WaitInput);
        }
    });
}
pub(super) fn ui_waitinput(mut contexts: EguiContexts) {
    egui::Window::new("WAIT INPUT").show(contexts.ctx_mut(), |_ui| {});
}

pub(super) fn transfer_input<T: Serialize + Sync + Send + 'static> (
    input: Res<GetInput<T>>,
    mut option_state: ResMut<NextState<OptionState>>,
    mut settings: ResMut<Settings>,
    mut command: Commands
) {
    if let Some(s) = input.to_settings().is_void() {
        *settings += s;
        command.remove_resource::<GetInput<T>>();
        option_state.set(OptionState::Option);
    }
}

#[derive(Resource)]
pub struct GetInput<T: Serialize> (pub T);


trait Transform {
    fn transform(&self) -> Result<String, Error>;
    fn to_settings(&self) -> Settings;
}

impl<G: 'static + Serialize> Transform for GetInput<G> {
    fn transform(&self) -> Result<String, Error> {
        ron::ser::to_string(&self.0)
    }

    fn to_settings(&self) -> Settings {
        let mut set = Settings { keyboard_input: HashMap::new(), mouse_input: HashMap::new(), mouse_motion: Some(Vec::new()) };
        match self.transform() {
            Ok(str) => {
                if let Ok(b) = ron::from_str::<MouseButton>(&str) {
                    set.mouse_input.insert(b, Movement::Void);
                }
                if let Ok(_) = ron::from_str::<MouseMotion>(&str) {
                    set.mouse_motion = Some(vec![Movement::Void]);
                }
                if let Ok(k) = ron::from_str::<KeyCode>(&str) {
                    set.keyboard_input.insert(k, Movement::Void);
                }
            },
            Err(_) => {},
        };
        set
        
    }
}
