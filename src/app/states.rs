use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States, Component)]
pub enum ClientAppState {
    #[default]
    Loading,
    MainMenu,
    GameMode,
}
