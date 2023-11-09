use std::collections::HashMap;
use std::hash::Hash;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
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

pub(super) fn transfer_input<T: Sync + Send + GetType + 'static> (
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

#[derive(Resource, Serialize, Deserialize)]
pub struct GetInput<T: Serialize> (pub T);


pub trait GetType where Self: Sized + Serialize + DeserializeOwned {
    fn get_type(&self) -> GetInputType;
    fn to_key(self) -> Option<KeyCode> {None}
    fn to_button(self) -> Option<MouseButton> {None}
    fn to_motion(self) -> Option<Motion> {None}
}
pub enum GetInputType {
    KeyCode,
    MouseButton,
    Motion
}

impl GetType for KeyCode {
    fn get_type(&self) -> GetInputType { GetInputType::KeyCode }
    fn to_key(self) -> Option<KeyCode> {Some(self)}
}
impl GetType for MouseButton {
    fn get_type(&self) -> GetInputType { GetInputType::MouseButton }
    fn to_button(self) -> Option<MouseButton> { Some(self) }
}
impl GetType for Motion {
    fn get_type(&self) -> GetInputType { GetInputType::Motion }
    fn to_motion(self) -> Option<Motion> { Some(self) }
}


impl<T: GetType> ToString for GetInput<T> {
    fn to_string(&self) -> String {
        ron::ser::to_string(&self).unwrap_or(String::from(""))
    }
}

trait Transform {
    fn to_settings(&self) -> Settings;
}

impl<T: GetType> Transform for GetInput<T> {
    fn to_settings(&self) -> Settings {
        let mut set = Settings { keyboard_input: HashMap::new(), mouse_input: HashMap::new(), mouse_motion: HashMap::new() };
        if let Ok(input) = ron::from_str::<T>(&self.to_string()) {
            match input.get_type() {
                GetInputType::KeyCode => set.keyboard_input.insert(input.to_key().unwrap(), Movement::Void),
                GetInputType::MouseButton => set.mouse_input.insert(input.to_button().unwrap(), Movement::Void),
                GetInputType::Motion => set.mouse_motion.insert(input.to_motion().unwrap(), Movement::Void),
            };
        }
        set
    }
}
