use std::hash::Hash;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use super::{MenuState, GeneralInput};

use crate::settings::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum OptionState {
    #[default]
    Option,
    WaitInput,
    WaitMovement,
    AddInput
}

pub(super) fn ui_options(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut option_state: ResMut<NextState<OptionState>>,
    mut commands: Commands,
    settings: Res<Settings>
) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Back").clicked() {
            // Save
            save_settings(&settings);
            menu_state.set(MenuState::Menu);
        }
        ui.separator();
        for (k, m) in settings.input.iter() {
            ui.horizontal(|ui| {
                if ui.button(m.to_string()).clicked() {
                    // set List Selector
                    commands.insert_resource(LastInput(k.clone()));
                    option_state.set(OptionState::WaitMovement);
                }

                if ui.button(format!("{:?}", k)).clicked() {
                    //save input to a Ressource
                    commands.insert_resource(LastInput(k.clone()));
                    option_state.set(OptionState::WaitInput);
                }
            });
        }
        ui.separator();
        if ui.button("+").clicked() {
            option_state.set(OptionState::WaitInput);
        }
    });
}
pub(super) fn ui_waitinput(mut contexts: EguiContexts) {
    egui::Window::new("WAIT INPUT").show(contexts.ctx_mut(), |_ui| {});
}

pub(super) fn ui_waitmovement(
    mut contexts: EguiContexts,
    mut option_state: ResMut<NextState<OptionState>>,
    mut settings: ResMut<Settings>,
    last_input: Option<Res<LastInput>>,
    mut commands: Commands
) {
    egui::Window::new("SET MOVEMENT").show(contexts.ctx_mut(), |ui| {
        Movement::iter().for_each(|m| {
            if ui.button(m.to_string()).clicked() {
                if let Some(last_in) = &last_input {
                    settings.input.insert(last_in.0.clone(), m);
                    commands.remove_resource::<LastInput>();
                }
                option_state.set(OptionState::Option);
            }
        });
    });
}

#[derive(Resource)]
pub(super) struct LastInput(pub GeneralInput);