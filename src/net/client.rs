use std::net::IpAddr;
use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode, connection::ConnectionConfiguration, Client,
    QuinnetClientPlugin,
};

use super::states::ServerConnectionState;

#[derive(Component)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetClientPlugin::default());
        app.add_systems(
            OnEnter(ServerConnectionState::NotHosting),
            (
                create_client_connection,
                // handle_server_messages // handles messages sent from the server to the client
            )
                .chain(),
        );
        app.add_systems(
            OnEnter(ServerConnectionState::Hosting),
            (
                create_client_connection,
                // handle_server_messages // handles messages sent from the server to the client
            )
                .chain(),
        );
        app.add_systems(
            OnEnter(ServerConnectionState::NoConnection),
            (
                close_client_connection,
                // handle_server_messages // handles messages sent from the server to the client
            )
                .chain(),
        );
    }
}

pub fn create_client_connection(mut client: ResMut<Client>) {
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

pub fn close_client_connection(mut client: ResMut<Client>) {
    client.close_all_connections().unwrap();
}
