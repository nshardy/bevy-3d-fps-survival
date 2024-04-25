use std::net::IpAddr;
use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServerPlugin;
use bevy_quinnet::server::{certificate::CertificateRetrievalMode, Server, ServerConfiguration};

use crate::ServerConnectionState;

#[derive(Component)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default());
        app.add_systems(
            OnEnter(ServerConnectionState::Hosting),
            (
                create_server,
                // handle_client_messages, // handles messages sent from client to server
            )
                .chain(),
        )
        .add_systems(
            OnExit(ServerConnectionState::Hosting),
            (
                close_server,
                // handle_client_messages, // handles messages sent from client to server
            )
                .chain(),
        );
    }
}

pub fn create_server(mut server: ResMut<Server>) {
    server
        .start_endpoint(
            ServerConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6000),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "Test".to_string(),
            },
        )
        .unwrap();
}

pub fn close_server(mut server: ResMut<Server>) {
    server.stop_endpoint().unwrap();
}
