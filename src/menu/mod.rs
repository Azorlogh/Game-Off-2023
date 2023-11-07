use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiContexts};
use bevy_inspector_egui::egui;
use bevy_rapier3d::prelude::{RapierConfiguration, TimestepMode};

use crate::GameState;



pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MenuState>()
        .add_systems(Update, ui_system.run_if(in_state(GameState::Menu)))
        .add_systems(Update, ui_pause_game.run_if(in_state(GameState::Pause)))
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
}

fn ui_system(mut contexts: EguiContexts, mut game_state: ResMut<NextState<GameState>>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        if ui.button("New Game").clicked() {
            game_state.set(GameState::Running);
        }
    });
}

fn ui_pause_game(mut contexts: EguiContexts) {
    egui::Window::new("Pause Game, Press Escape").show(contexts.ctx_mut(), |ui| {
    });
}