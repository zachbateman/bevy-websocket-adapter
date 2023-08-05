use crate::shared::{ConnectionHandle, Enveloppe, GenericParser, NetworkEvent, NetworkEventHolder, GenericParserHolder};
use crate::server::Server;
use bevy::prelude::*;
use log::{trace, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::ConnEnvMap;

#[derive(Default, Debug)]
pub struct WebSocketServer {}

impl Plugin for WebSocketServer {
    fn build(&self, app: &mut App) {
        let server = Server::new();
        let router = Arc::new(Mutex::new(GenericParser::new()));
        let router_holder = GenericParserHolder { parser: router };
        let map = HashMap::<String, Vec<(ConnectionHandle, Enveloppe)>>::new();
        let map_holder = ConnEnvMap { map };
        let network_events = Vec::<NetworkEvent>::new();
        let network_event_holder = NetworkEventHolder { events: network_events };
        app.insert_resource(server)
            .insert_resource(router_holder)
            .insert_resource(map_holder)
            .insert_resource(network_event_holder)
            .add_event::<NetworkEvent>()
            .add_systems(Update, consume_messages)
            .add_systems(Update, super::shared::handle_network_events);
    }
}

fn consume_messages(
    server: Res<Server>,
    mut hmap: ResMut<ConnEnvMap>,
    mut network_events: ResMut<NetworkEventHolder>,
) {
    if !server.is_running() {
        return;
    }

    while let Some(ev) = server.recv() {
        match ev {
            NetworkEvent::Message(handle, raw_ev) => {
                trace!("consuming message from {:?}", handle);
                if let Ok(enveloppe) = serde_json::from_reader::<std::io::Cursor<Vec<u8>>, Enveloppe>(
                    std::io::Cursor::new(raw_ev),
                ) {
                    let tp = enveloppe.message_type.to_string();
                    let mut v = if let Some(x) = hmap.remove(&tp) {
                        x
                    } else {
                        Vec::new()
                    };
                    v.push((handle, enveloppe.clone()));
                    hmap.insert(tp, v);
                } else {
                    warn!("failed to deserialize message from {:?}", handle);
                    continue;
                }
            }
            other => {
                trace!("received network event: {:?}", other);
                network_events.push(other);
            }
        }
    }
}

