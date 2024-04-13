use std::net::IpAddr;
use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_quinnet::{
    client::{
        certificate::CertificateVerificationMode, connection::ConnectionConfiguration, Client,
        QuinnetClientPlugin,
    },
    server::{
        certificate::CertificateRetrievalMode, QuinnetServerPlugin, Server, ServerConfiguration,
    },
};

use states::*;
mod states;

fn main() {
    App::new()
        // states
        .init_state::<ClientAppState>()
        .init_state::<ClientGameState>()
        .init_state::<ClientConnectionState>()
        .init_state::<ClientPauseState>()
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_plugins(QuinnetServerPlugin::default())
        .add_plugins(QuinnetClientPlugin::default())
        // state dependent systems
        .add_systems(OnEnter(ClientAppState::Loading), loading_state)
        .add_systems(OnEnter(ClientAppState::MainMenu), main_menu_state)
        .add_systems(
            OnEnter(ClientConnectionState::Hosting),
            (
                create_server,
                create_client,
                // handle_client_messages, // handles messages sent from client to server
            )
                .chain(),
        )
        .add_systems(
            OnEnter(ClientConnectionState::NotHosting),
            (
                create_client,
                // handle_server_messages // handles messages sent from the server to the client
            )
                .chain(),
        )
        // .add_systems(OnExit(Connection::Hosting))
        .add_systems(
            Update,
            (pause_state).run_if(in_state(ClientAppState::InGame)),
        )
        // state-independent systems
        .add_systems(Startup, setup)
        // run
        .run();
}

// before going to the main menu
fn loading_state(
    // mut asset_server: Res<AssetServer>,
    mut next_app_state: ResMut<NextState<ClientAppState>>,
    mut next_game_state: ResMut<NextState<ClientGameState>>,
    mut next_connection_state: ResMut<NextState<ClientConnectionState>>,
) {
    // after finishing loading assets
    next_app_state.set(ClientAppState::MainMenu);
    next_game_state.set(ClientGameState::NotInGame);
    next_connection_state.set(ClientConnectionState::NoConnection);
    //
    // Hosting Test
    // next_app_state.set(AppState::InGame);
    // next_game_state.set(GameState::Game);
    // next_connection_state.set(ConnectionState::Hosting);
    //
    // Not Hosting Test
    // next_app_state.set(AppState::InGame);
    // next_game_state.set(GameState::Game);
    // next_connection_state.set(ConnectionState::NotHosting);
}

// main menu
fn main_menu_state(
    mut next_app_state: ResMut<NextState<ClientAppState>>,
    mut next_game_state: ResMut<NextState<ClientGameState>>,
) {
    next_app_state.set(ClientAppState::MainMenu);
    next_game_state.set(ClientGameState::NotInGame);
}

// joining multiplayer
fn joining_multiplayer_state(
    mut next_app_state: ResMut<NextState<ClientAppState>>,
    mut next_game_state: ResMut<NextState<ClientGameState>>,
    mut next_connection_state: ResMut<NextState<ClientConnectionState>>,
) {
    next_app_state.set(ClientAppState::InGame);
    next_game_state.set(ClientGameState::Game);
    next_connection_state.set(ClientConnectionState::NotHosting);
}

fn create_client(mut client: ResMut<Client>) {
    client
        .open_connection(
            ConnectionConfiguration::from_ips(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                6000,
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                0,
            ),
            CertificateVerificationMode::SkipVerification,
        )
        .unwrap();
}

// hosting multiplayer
fn hosting_multiplayer_state(
    mut next_app_state: ResMut<NextState<ClientAppState>>,
    mut next_game_state: ResMut<NextState<ClientGameState>>,
    mut next_connection_state: ResMut<NextState<ClientConnectionState>>,
) {
    next_app_state.set(ClientAppState::InGame);
    next_game_state.set(ClientGameState::Game);
    next_connection_state.set(ClientConnectionState::Hosting);
}

fn create_server(mut server: ResMut<Server>) {
    server
        .start_endpoint(
            ServerConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6000),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "Test".to_string(),
            },
        )
        .unwrap();
}

// in_game: pausing
fn pause_state(
    pause_state: Res<State<ClientPauseState>>,
    keys: ResMut<ButtonInput<KeyCode>>,
    mut next_pause_state: ResMut<NextState<ClientPauseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match pause_state.get() {
            ClientPauseState::Paused => next_pause_state.set(ClientPauseState::NotPaused),
            ClientPauseState::NotPaused => next_pause_state.set(ClientPauseState::Paused),
        }
    }
}

// independent functions
fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
