use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, LockResult, MutexGuard};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Enveloppe {
    #[serde(rename(serialize = "t", deserialize = "t"))]
    pub message_type: String,
    #[serde(rename(serialize = "d", deserialize = "d"))]
    pub payload: Box<serde_json::value::RawValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendEnveloppe<T> {
    #[serde(rename(serialize = "t", deserialize = "t"))]
    pub message_type: String,
    #[serde(rename(serialize = "d", deserialize = "d"))]
    pub payload: T,
}

pub trait MessageType: Any + serde::de::DeserializeOwned + Send + Sync {
    fn message_type() -> &'static str;
}

type Df = Box<dyn Send + Fn(&serde_json::value::RawValue) -> anyhow::Result<Box<dyn Any + Send>>>;

fn generate_deserialize_fn<T: Any>() -> Df
where
    T: serde::de::DeserializeOwned + Send,
{
    Box::new(|v: &serde_json::value::RawValue| Ok(Box::new(serde_json::from_str::<T>(v.get())?)))
}


#[derive(Resource)]
pub struct GenericParserHolder {
    pub(crate) parser: Arc<Mutex<GenericParser>>,
}
impl GenericParserHolder {
    pub fn lock(&self) -> LockResult<MutexGuard<'_, GenericParser>> {
        self.parser.lock()
    }
}


#[derive(Default)]
pub struct GenericParser {
    tps: HashMap<String, Box<dyn Any + Send>>,
}

impl GenericParser {
    pub fn new() -> Self {
        Self {
            tps: HashMap::new(),
        }
    }

    pub fn insert_type<T: MessageType>(&mut self) {
        let tag = T::message_type();
        if self.tps.get(tag).is_some() {
            panic!("type '{}' already registered", tag);
        }
        self.tps
            .insert(tag.to_string(), Box::new(generate_deserialize_fn::<T>()));
    }

    pub fn parse_as_any(
        &self,
        tag: &str,
        dat: &serde_json::value::RawValue,
    ) -> anyhow::Result<Box<dyn Any + Send>> {
        match self.tps.get(tag) {
            Some(func) => {
                return func
                    .downcast_ref::<Df>()
                    .expect("failed to load downcast function")(dat);
            }
            None => anyhow::bail!("type '{}' not registered", tag),
        }
    }

    pub fn parse_enveloppe(&self, ev: &Enveloppe) -> anyhow::Result<Box<dyn Any + Send>> {
        self.parse_as_any(&ev.message_type, &ev.payload)
    }

    pub fn try_into_concrete_type<T: 'static>(d: Box<dyn Any + Send>) -> anyhow::Result<T> {
        match d.downcast::<T>() {
            Ok(r) => Ok(*r),
            Err(_) => anyhow::bail!("downcast type mismatch"),
        }
    }
}
