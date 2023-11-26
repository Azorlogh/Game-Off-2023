use bevy::prelude::*;

#[derive(Event)]
enum UiMessage {}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MenuState {
	#[default]
	Menu,
	Option,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum OptionState {
	#[default]
	Option,
}
