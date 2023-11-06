use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MenuState>()
        ;
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MenuState {
    #[default]
    Menu
}