extern crate bevy_websocket_adapter;
use ::bevy::prelude::*;
use bevy_websocket_adapter::{
    bevy::{WebSocketServer, WsMessageInserter, ConnHandleEvent},
    impl_message_type,
    server::Server,
};
use log::info;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ping {}
impl_message_type!(Ping, "ping");

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pong {}
impl_message_type!(Pong, "pong");

fn start_listen(mut ws: ResMut<Server>) {
    ws.listen("0.0.0.0:12345")
        .expect("failed to start websocket server");
}

fn respond_to_pings(mut evs: EventReader<ConnHandleEvent<Ping>>, srv: Res<Server>) {
    for conn_handle in evs.iter() {
        info!("received ping from {:?} : {:?}", conn_handle.handle, conn_handle.msg);
        srv.send_message(&conn_handle.handle, &Pong {})
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(WebSocketServer::default())
        .add_systems(Startup, start_listen)
        .add_message_type::<Ping>()
        .add_systems(Update, respond_to_pings)
        .run();
}
