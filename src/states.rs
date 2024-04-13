use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientAppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientGameState {
    #[default]
    NotInGame,
    Game,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientConnectionState {
    #[default]
    NoConnection,
    NotHosting,
    Hosting,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientPauseState {
    #[default]
    NotPaused,
    Paused,
}
