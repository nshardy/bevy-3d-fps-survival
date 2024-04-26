use bevy::prelude::*;

use crate::app::states::ClientAppState;

#[derive(Component)]
pub struct FpsText;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct LoadingUIPlugin;

impl Plugin for LoadingUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ClientAppState::Loading), setup_loading)
            .add_systems(OnExit(ClientAppState::Loading), remove_loading);
    }
}

fn remove_loading(mut commands: Commands, mut text_query: Query<Entity, With<Text>>) {
    for entity in text_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn setup_loading(mut commands: Commands, mut next_app_state: ResMut<NextState<ClientAppState>>) {
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_section(
            "Loading...",
            TextStyle {
                font_size: 60.0,
                color: Color::rgba(1.0, 1.0, 1.0, 0.8),
                ..default()
            },
        )
        .with_style(Style {
            display: Display::Flex,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..default()
        }),
        FpsText,
    ));

    next_app_state.set(ClientAppState::MainMenu);
}
