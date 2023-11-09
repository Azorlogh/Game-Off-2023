use bevy::{prelude::*, app::AppExit};
use bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;

use crate::GameState;

mod setting;
mod input_tranformer;
pub use setting::*;
pub use input_tranformer::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MenuState>()
        .add_state::<OptionState>()
        .add_systems(Update, ui_system.run_if(in_state(GameState::Menu).and_then(in_state(MenuState::Menu))))
        .add_systems(Update, ui_pause_game.run_if(in_state(GameState::Pause).and_then(in_state(MenuState::Menu))))
        .add_systems(Update, ui_options.run_if(in_state(MenuState::Option)))
        .add_systems(OnEnter(OptionState::AddInput), transfer_input)
        .add_systems(Update, ui_waitinput.run_if(in_state(OptionState::WaitInput)))
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