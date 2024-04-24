use std::net::IpAddr;
use std::net::Ipv4Addr;

use bevy::ecs::system::ResMut;
use bevy_quinnet::server::{certificate::CertificateRetrievalMode, Server, ServerConfiguration};

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
