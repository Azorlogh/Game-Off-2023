use std::hash::Hash;

use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_egui::{EguiContexts, egui};
use super::MenuState;

use crate::settings::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum OptionState {
    #[default]
    Option,
}

pub(super) fn ui_options(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<NextState<MenuState>>,

    mut settings: ResMut<Settings>,
    mut changing_movement: Local<Option<Movement>>,

    keys: Res<Input<KeyCode>>,
	buttons: Res<Input<MouseButton>>,
	mut motion: EventReader<MouseMotion>,
) {
    if let Some(movement) = *changing_movement {
        let delta = motion.read().fold(Vec2::ZERO, |acc, x| acc + x.delta);

        let mut general_input = None;

        for k in keys.get_just_pressed() {
            general_input = Some(GeneralInput::KeyCode(*k));
        }

        for b in buttons.get_just_pressed() {
            general_input = Some(GeneralInput::MouseButton(*b));
        }

        if delta.length() > 10.0 {
            general_input = Some(GeneralInput::Motion);
        }

        if let Some(input) = general_input {
            *settings.input.get_mut(&movement).unwrap() = input;
            *changing_movement = None;
        }
    }

    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Back").clicked() {
            // Save
            save_settings(&settings);
            menu_state.set(MenuState::Menu);
        }
        ui.separator();
        for (m, input) in settings.input.iter() {
            ui.horizontal(|ui| {
                ui.label(m.to_string());
                ui.horizontal(|ui_hor| {
                    if ui_hor.button(format!("{:?}", input)).clicked() {
                        *changing_movement = Some(*m);
                    }
                });
            });
        }
    });
}