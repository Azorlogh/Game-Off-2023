use bevy::prelude::*;
use bevy_iced::{IcedPlugin, IcedContext, iced::widget::text};

use crate::GameState;


pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(IcedPlugin::default())
        .add_event::<UiMessage>()
        .add_state::<MenuState>()
        .add_systems(Update, ui_system.run_if(in_state(GameState::Menu)).run_if(in_state(MenuState::Menu)))
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
    InGame
}


fn ui_system(time: Res<Time>, mut ctx: IcedContext<UiMessage>) {
    ctx.display(text(format!(
        "Hello Iced! Running for {:.2} seconds.",
        time.elapsed_seconds()
    )));
}