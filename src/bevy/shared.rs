use crate::shared::{ConnectionHandle, Enveloppe, GenericParser, MessageType, NetworkEvent, NetworkEventHolder, GenericParserHolder};
use bevy::prelude::*;
use log::warn;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


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
    // mut events: ResMut<Vec<NetworkEvent>>,
    mut events: ResMut<NetworkEventHolder>,
    mut sink: EventWriter<NetworkEvent>,
) {
    // let x = Vec::new();
    // x.drain(..);
    // for ev in events.drain(..) {
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
    value: String,
) -> impl FnMut(
// pub(crate) fn add_message_consumer<T>(
    // key: Local<String>,
    Local<String>,
    // mut hmap: ResMut<HashMap<String, Vec<(ConnectionHandle, Enveloppe)>>>,
    // mut hmap: ResMut<ConnEnvMap>,
    ResMut<ConnEnvMap>,
    // router: Res<Arc<Mutex<GenericParser>>>,
    // router: Res<GenericParserHolder>,
    Res<GenericParserHolder>,
    // mut queue: EventWriter<(ConnectionHandle, T)>,
    // mut queue: EventWriter<ConnHandleEvent<T>>,
    EventWriter<ConnHandleEvent<T>>,
    // Using Comands instead of EventWriter for arbitrary type.
    // See: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.EventWriter.html#limitations
    // mut commands: Commands,  
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
                            // queue.send((handle, msg));
                            // commands.add(|w: &mut World| {
                            //     w.send_event((handle, msg));
                            // });
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

// // pub fn add_message_consumer_wrapper<T>(x: T::message_type().to_string())
// pub fn add_message_consumer_wrapper<T>(x: String)
//     where
//     T: MessageType + 'static,
//     {
//         move || {
//             add_message_consumer(key, hmap, router, queue)
//         }
// }




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
        // self.add_event::<(ConnectionHandle, T)>();
        self.add_event::<ConnHandleEvent<T>>();
        let router = self
            // .app
            .world
            // .get_resource::<Arc<Mutex<GenericParser>>>()
            .get_resource::<GenericParserHolder>()
            .expect("cannot register message before WebSocketServer initialization");
        router.lock().unwrap().insert_type::<T>();



        // self.add_system(add_message_consumer::<T>.system().config(|params| {
        // self.add_systems(Update, add_message_consumer::<T>.config(|params| {
        //     params.0 = Some(T::message_type().to_string());
        // }));

        self.add_systems(Update, add_message_consumer_configured::<T>(T::message_type().to_string()));




        self
    }
}
