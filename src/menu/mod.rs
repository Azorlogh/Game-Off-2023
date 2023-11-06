use bevy::prelude::*;
use bevy_asset_loader::loading_state::LoadingStateAppExt;

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