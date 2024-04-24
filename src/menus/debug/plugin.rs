use bevy::prelude::*;
use bevy_inspector_egui::{self, quick::StateInspectorPlugin};

use crate::app::states::{ClientAppState, ClientConnectionState, ClientGameState};

#[derive(Component)]
pub struct CustomInspectorPlugin;

impl Plugin for CustomInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StateInspectorPlugin::<ClientAppState>::default())
            .add_plugins(StateInspectorPlugin::<ClientGameState>::default())
            .add_plugins(StateInspectorPlugin::<ClientConnectionState>::default());
    }
}
