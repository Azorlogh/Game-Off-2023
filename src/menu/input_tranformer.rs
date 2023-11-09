use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::settings::*;

use super::OptionState;

pub(super) fn transfer_input<T: Sync + Send + GetType + 'static> (
    input: Res<GetInput<T>>,
    mut option_state: ResMut<NextState<OptionState>>,
    mut settings: ResMut<Settings>,
    mut command: Commands
) {
    println!("{:?}", input.to_settings());
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
    fn to_key(&self) -> Option<KeyCode> {None}
    fn to_button(&self) -> Option<MouseButton> {None}
    fn to_motion(&self) -> Option<Motion> {None}
}
pub enum GetInputType {
    KeyCode,
    MouseButton,
    Motion
}

impl GetType for KeyCode {
    fn get_type(&self) -> GetInputType { GetInputType::KeyCode }
    fn to_key(&self) -> Option<KeyCode> {Some(*self)}
}
impl GetType for MouseButton {
    fn get_type(&self) -> GetInputType { GetInputType::MouseButton }
    fn to_button(&self) -> Option<MouseButton> { Some(*self) }
}
impl GetType for Motion {
    fn get_type(&self) -> GetInputType { GetInputType::Motion }
    fn to_motion(&self) -> Option<Motion> { Some(*self) }
}

impl<T: GetType> GetInput<T> {
    fn to_settings(&self) -> Settings {
        let mut set = Settings { keyboard_input: HashMap::new(), mouse_input: HashMap::new(), mouse_motion: HashMap::new() };
        match self.0.get_type() {
            GetInputType::KeyCode => set.keyboard_input.insert(self.0.to_key().unwrap(), Movement::Void),
            GetInputType::MouseButton => set.mouse_input.insert(self.0.to_button().unwrap(), Movement::Void),
            GetInputType::Motion => set.mouse_motion.insert(self.0.to_motion().unwrap(), Movement::Void),
        };
        
        set
    }
}
