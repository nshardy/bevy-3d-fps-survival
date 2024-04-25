use bevy::prelude::*;

use app::states::*;
use menus::{
    debug::plugin::CustomInspectorPlugin, loading_assets::plugin::LoadingUIPlugin,
    main_menu::plugin::MainMenuUIPlugin,
};
use net::{client::*, server::ServerPlugin};

mod app;
mod menus;
mod net;

fn main() {
    let mut app = App::new();
    // resources
    app.insert_resource(ClearColor(Color::ALICE_BLUE));

    // states
    app.init_state::<ClientAppState>()
        .init_state::<ClientGameState>()
        .init_state::<ServerConnectionState>();

    // plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(CustomInspectorPlugin)
        .add_plugins(ClientPlugin)
        .add_plugins(ServerPlugin)
        .add_plugins(LoadingUIPlugin)
        .add_plugins(MainMenuUIPlugin);

    // run
    app.run();
}
