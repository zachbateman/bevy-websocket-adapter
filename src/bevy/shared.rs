use crate::shared::{ConnectionHandle, Enveloppe, GenericParser, MessageType, NetworkEvent, NetworkEventHolder, GenericParserHolder};
use bevy::prelude::*;
use log::warn;
use std::collections::HashMap;


#[derive(Resource)]
pub struct ConnEnvMap {
    pub map: HashMap<String, Vec<(ConnectionHandle, Enveloppe)>>,
}
impl ConnEnvMap {
    pub fn insert(&mut self, k: String, v: Vec<(ConnectionHandle, Enveloppe)>) {
        self.map.insert(k, v);
    }
    pub fn remove(&mut self, k: &String) -> Option<Vec<(ConnectionHandle, Enveloppe)>> {
        self.map.remove(k)
    }
}




pub(crate) fn handle_network_events(
    mut events: ResMut<NetworkEventHolder>,
    mut sink: EventWriter<NetworkEvent>,
) {
    for ev in events.drain() {
        sink.send(ev);
    }
}

#[derive(Event)]
pub struct ConnHandleEvent<T> {
    pub handle: ConnectionHandle,
    pub msg: T,
}


pub(crate) fn add_message_consumer_configured<T>(
) -> impl FnMut(
    Local<String>,
    ResMut<ConnEnvMap>,
    Res<GenericParserHolder>,
    EventWriter<ConnHandleEvent<T>>,
) where
    T: Send + Sync + 'static,
{
    move |key, mut hmap, router, mut queue| {
        if let Some(values) = hmap.remove(&*key) {
            for (handle, v) in values {
                let enveloppe = router.lock().unwrap().parse_enveloppe(&v);
                match enveloppe {
                    Ok(dat) => match GenericParser::try_into_concrete_type::<T>(dat) {
                        Ok(msg) => {
                            let event = ConnHandleEvent { handle, msg };
                            queue.send(event);
                        }
                        Err(e) => {
                            warn!("failed to downcast : {}", e);
                        }
                    },
                    Err(e) => {
                        warn!("failed to parse type enveloppe : {}", e);
                        continue;
                    }
                };
            }
        }
    }
}




pub trait WsMessageInserter {
    #[deprecated(
        since = "0.1.4",
        note = "Use [`add_message_type`](#method.add_message_type) instead."
    )]
    fn register_message_type<T>(&mut self) -> &mut Self
    where
        T: MessageType + 'static,
    {
        self.add_message_type::<T>()
    }
    fn add_message_type<T>(&mut self) -> &mut Self
    where
        T: MessageType + 'static;
}

impl WsMessageInserter for App {
    fn add_message_type<T>(&mut self) -> &mut Self
    where
        T: MessageType + 'static,
    {
        self.add_event::<ConnHandleEvent<T>>();
        let router = self
            .world
            .get_resource::<GenericParserHolder>()
            .expect("cannot register message before WebSocketServer initialization");
        router.lock().unwrap().insert_type::<T>();

        self.add_systems(Update, add_message_consumer_configured::<T>());

        self
    }
}
