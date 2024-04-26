use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States, Component)]
pub enum ClientGameState {
    #[default]
    NotInGame,
    InGame,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States, Component)]
pub enum ServerConnectionState {
    #[default]
    NoConnection,
    NotHosting,
    Hosting,
}
