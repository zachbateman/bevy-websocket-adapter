extern crate bevy_websocket_adapter;
use ::bevy::prelude::*;
use bevy_websocket_adapter::{
    bevy::WebSocketServer,
    shared::NetworkEvent,
    server::Server,
};
use log::info;

fn start_listen(mut ws: ResMut<Server>) {
    ws.listen("0.0.0.0:12345")
        .expect("failed to start websocket server");
}

fn listen_for_events(mut evs: EventReader<NetworkEvent>) {
    for ev in evs.iter() {
        info!("received NetworkEvent : {:?}", ev);
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    // App::build()
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(WebSocketServer::default())
        // .add_startup_system(start_listen.system())
        .add_systems(Startup, start_listen)
        // .add_system(listen_for_events.system())
        .add_systems(Update, listen_for_events)
        .run();
}
