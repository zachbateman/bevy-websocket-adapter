use std::vec::Drain;

use bevy::prelude::{Resource, Event};
use thiserror::Error as TError;
use super::ConnectionHandle;

#[derive(TError, Debug)]
pub enum NetworkError {}

#[derive(Debug, Resource, Event)]
pub enum NetworkEvent {
    Connected(ConnectionHandle),
    Disconnected(ConnectionHandle),
    Message(ConnectionHandle, Vec<u8>),
    Error(Option<ConnectionHandle>, anyhow::Error),
}

#[derive(Debug, Resource)]
pub struct NetworkEventHolder {
    pub(crate) events: Vec<NetworkEvent>,
}
impl NetworkEventHolder {
    pub fn push(&mut self, event: NetworkEvent) {
        self.events.push(event);
    }
    pub fn drain(&mut self) -> Drain<'_, NetworkEvent> {
        self.events.drain(..)
    }
}