use bevy::{app::AppExit, prelude::*};

use crate::app::states::ClientAppState;
use crate::net::states::{ClientGameState, ServerConnectionState};

#[derive(Component)]
pub struct FpsText;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct MainMenuUIPlugin;

impl Plugin for MainMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ClientAppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(ClientAppState::MainMenu), remove_main_menu)
            .add_systems(
                Update,
                (menu_action).run_if(in_state(ClientAppState::MainMenu)),
            );
    }
}

#[derive(Component)]
pub enum MenuButtonAction {
    Singleplayer,
    Multiplayer,
    Quit,
}

fn remove_main_menu(
    mut commands: Commands,
    mut query: Query<Entity, Or<(With<Text>, With<Camera2d>, With<Button>)>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.insert_resource(ClearColor(Color::ALICE_BLUE));

    commands.spawn((
        TextBundle::from_section(
            "3D FPS Survival",
            TextStyle {
                font_size: 60.0,
                color: Color::BLACK,
                ..default()
            },
        )
        .with_style(Style {
            display: Display::Flex,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(25.0),
                ..default() // bottom: Val::Auto,
            },
            ..default()
        }),
        FpsText,
    ));

    // Singleplayer button
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(150.0),
                        ..default() // bottom: Val::Auto,
                    },
                    padding: UiRect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            MenuButtonAction::Singleplayer,
        ))
        .with_children(|button_parent| {
            button_parent.spawn(TextBundle::from_section(
                "Singleplayer",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });

    // Multiplayer button
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(200.0),
                        ..default() // bottom: Val::Auto,
                    },
                    padding: UiRect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            MenuButtonAction::Multiplayer,
        ))
        .with_children(|button_parent| {
            button_parent.spawn(TextBundle::from_section(
                "Multiplayer",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });

    // quit button
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(250.0),
                        ..default() // bottom: Val::Auto,
                    },
                    padding: UiRect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            MenuButtonAction::Quit,
        ))
        .with_children(|button_parent| {
            button_parent.spawn(TextBundle::from_section(
                "Quit",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_app_state: ResMut<NextState<ClientAppState>>,
    mut next_game_state: ResMut<NextState<ClientGameState>>,
    mut next_connection_state: ResMut<NextState<ServerConnectionState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Singleplayer => {
                    next_app_state.set(ClientAppState::GameMode);
                    next_game_state.set(ClientGameState::InGame);
                    next_connection_state.set(ServerConnectionState::Hosting);
                }
                MenuButtonAction::Multiplayer => {
                    println!("hi there, not implemented")
                    // next_app_state.set(client::states::ClientAppState::GameMode);
                    // next_game_state.set(client::states::ClientGameState::InGame);
                    // next_game_state.set(client::states::ClientGameState::InGame);
                }
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}
