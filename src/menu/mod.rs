use bevy::prelude::*;

use crate::GameState;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MenuState>()
        .add_systems(OnExit(GameState::Menu), clearing)
        ;
    }
}

fn clearing() {

}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MenuState {
    #[default]
    Menu
}