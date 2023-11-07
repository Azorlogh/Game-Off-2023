use bevy::{prelude::*, app::AppExit};
use bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;

use crate::GameState;



pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MenuState>()
        .add_systems(Update, ui_system.run_if(in_state(GameState::Menu).and_then(in_state(MenuState::Menu))))
        .add_systems(Update, ui_pause_game.run_if(in_state(GameState::Pause).and_then(in_state(MenuState::Menu))))
        .add_systems(Update, ui_options.run_if(in_state(MenuState::Option)))
        ;
    }
}

#[derive(Event)]
enum UiMessage {
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MenuState {
    #[default]
    Menu,
    Option
}

fn ui_system(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_exit_events: ResMut<Events<AppExit>>
) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("New Game").clicked() {
            game_state.set(GameState::Running);
        }
        if ui.button("Options").clicked() {
            menu_state.set(MenuState::Option);
        }
        if ui.button("Quit").clicked() {
            app_exit_events.send(AppExit)
        }
    });
}

fn ui_pause_game(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_exit_events: ResMut<Events<AppExit>>
) {
    egui::Window::new("Pause Game, Press Escape").show(contexts.ctx_mut(), |ui| {
        if ui.button("Resume").clicked() {
            game_state.set(GameState::Running);
        }
        if ui.button("Options").clicked() {
            menu_state.set(MenuState::Option);
        }
        if ui.button("Quit").clicked() {
            app_exit_events.send(AppExit)
        }
    });
}

fn ui_options(mut contexts: EguiContexts, mut menu_state: ResMut<NextState<MenuState>>) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Back").clicked() {
            menu_state.set(MenuState::Menu);
        }

        if ui.button("Apply").clicked() {

        }
    });
}