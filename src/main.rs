use bevy::prelude::*;
use bevy_quinnet::{client::QuinnetClientPlugin, server::QuinnetServerPlugin};

mod app;
mod menus;
mod net;

fn main() {
    let mut app = App::new();
    // resources
    app.insert_resource(ClearColor(Color::ALICE_BLUE));

    // states
    app.init_state::<app::states::ClientAppState>()
        .init_state::<app::states::ClientGameState>()
        .init_state::<app::states::ClientConnectionState>();
    // .init_state::<app::states::ClientPauseState>();

    // plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(menus::debug::plugin::CustomInspectorPlugin)
        .add_plugins(QuinnetServerPlugin::default())
        .add_plugins(QuinnetClientPlugin::default())
        .add_plugins(menus::loading_assets::plugin::LoadingUIPlugin)
        .add_plugins(menus::main_menu::plugin::MainMenuUIPlugin);

    // state dependent systems
    app.add_systems(
        OnEnter(app::states::ClientConnectionState::Hosting),
        (
            net::server::create_server,
            net::client::create_client_connection,
            // net::client::handle_client_messages, // handles messages sent from client to server
        )
            .chain(),
    )
    .add_systems(
        OnExit(app::states::ClientConnectionState::Hosting),
        (
            net::server::close_server,
            net::client::close_client_connection,
            // net::client::handle_client_messages, // handles messages sent from client to server
        )
            .chain(),
    )
    .add_systems(
        OnEnter(app::states::ClientConnectionState::NotHosting),
        (
            net::client::create_client_connection,
            // net::server::handle_server_messages // handles messages sent from the server to the client
        )
            .chain(),
    )
    .add_systems(
        OnEnter(app::states::ClientConnectionState::NoConnection),
        (
            net::client::close_client_connection,
            // net::server::handle_server_messages // handles messages sent from the server to the client
        )
            .chain(),
    );

    // run
    app.run();
}
