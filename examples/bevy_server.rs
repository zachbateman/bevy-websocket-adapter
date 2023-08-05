extern crate bevy_websocket_adapter;
use ::bevy::prelude::*;
use bevy_websocket_adapter::{
    bevy::{WebSocketServer, WsMessageInserter, ConnHandleEvent},
    impl_message_type,
    shared::ConnectionHandle,
    server::Server,
};
use log::info;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DummyEvent {
    a: u32,
}
impl_message_type!(DummyEvent, "dummy");

fn start_listen(mut ws: ResMut<Server>) {
    ws.listen("0.0.0.0:12345")
        .expect("failed to start websocket server");
}

// fn listen_for_dummy(mut evs: EventReader<(ConnectionHandle, DummyEvent)>) {
fn listen_for_dummy(mut evs: EventReader<ConnHandleEvent<DummyEvent>>) {
    // for (handle, ev) in evs.iter() {
    for conn_handle in evs.iter() {
        // info!("received DummyEvent from {:?} : {:?}", handle, ev);
        info!("received DummyEvent from {:?} : {:?}", conn_handle.handle, conn_handle.msg);
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
        .add_message_type::<DummyEvent>()
        // .add_system(listen_for_dummy.system())
        .add_systems(Update, listen_for_dummy)
        .run();
}
