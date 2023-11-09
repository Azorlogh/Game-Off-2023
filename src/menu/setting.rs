use std::hash::Hash;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use super::MenuState;

use crate::settings::*;
use crate::util::VecExt;

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
            // Save
            menu_state.set(MenuState::Menu);
        }
        for (k, m) in settings.keyboard_input.iter().map(|(k, m)| (k, m)).collect::<Vec<(&KeyCode, &Movement)>>().sorted() {
            if ui.button(format!("{:?}", k)).clicked() {
                option_state.set(OptionState::WaitInput);
                // wait input
            }

            if ui.button(m.to_string()).clicked() {
                // set List Selector
            }
        }
        for (k, m) in settings.mouse_input.iter() {
            if ui.button(format!("{:?}", k)).clicked() {
                option_state.set(OptionState::WaitInput);
                // wait input
            }

            if ui.button(m.to_string()).clicked() {
                // set List Selector
            }
        }
        for (k, m) in settings.mouse_motion.iter().map(|(k, m)| (k, m)).collect::<Vec<(&Motion, &Movement)>>().sorted() {
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
