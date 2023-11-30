use bevy::{app::AppExit, prelude::*};
use bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;

use crate::{game::GameState, AppState};

use crate::settings::{Action, GeneralInput, Settings};

use crate::settings::systems::save_settings;

use super::MenuState;
use bevy::input::mouse::MouseMotion;

pub fn ui_options(
	mut contexts: EguiContexts,
	mut menu_state: ResMut<NextState<MenuState>>,

	mut settings: ResMut<Settings>,
	mut changing_movement: Local<Option<Action>>,

	keys: Res<Input<KeyCode>>,
	buttons: Res<Input<MouseButton>>,
	mut motion: EventReader<MouseMotion>,
) {
	if let Some(movement) = *changing_movement {
		let delta = motion.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);

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

pub fn ui_system(
	mut contexts: EguiContexts,
	mut game_state: ResMut<NextState<GameState>>,
	mut app_state: ResMut<NextState<AppState>>,
	mut menu_state: ResMut<NextState<MenuState>>,
	mut app_exit_events: ResMut<Events<AppExit>>,
) {
	egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
		if ui.button("New Game").clicked() {
			game_state.set(GameState::Playing);
		}
		if ui.button("Options").clicked() {
			menu_state.set(MenuState::Option);
		}
		if ui.button("Back to Main Menu").clicked() {
			app_state.set(AppState::MainMenu);
		}
		if ui.button("Quit").clicked() {
			app_exit_events.send(AppExit)
		}
	});
}

pub fn ui_pause_game(
	mut contexts: EguiContexts,
	mut game_state: ResMut<NextState<GameState>>,
	mut app_state: ResMut<NextState<AppState>>,
	mut menu_state: ResMut<NextState<MenuState>>,
	mut app_exit_events: ResMut<Events<AppExit>>,
) {
	egui::Window::new("Pause Game, Press Escape").show(contexts.ctx_mut(), |ui| {
		if ui.button("Resume").clicked() {
			game_state.set(GameState::Playing);
		}
		if ui.button("Options").clicked() {
			menu_state.set(MenuState::Option);
		}
		if ui.button("Back to Main Menu").clicked() {
			app_state.set(AppState::MainMenu);
		}
		if ui.button("Quit").clicked() {
			app_exit_events.send(AppExit)
		}
	});
}
