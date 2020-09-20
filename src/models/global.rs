use std::collections::HashSet;

use crate::events;
use crate::models::object::Object;

pub struct ObjectStore<'a> {
    pub alive: &'a mut HashSet<Object>,
    pub removed: &'a mut HashSet<Object>,
}

pub struct EventStore<'a> {
    pub global: &'a mut HashSet<events::global::GlobalEvent>,
    pub object: &'a mut HashSet<events::object::ObjectEvent>,
}

pub struct Global<'a> {
    pub date: u64,

    pub object: ObjectStore<'a>,

    pub event: EventStore<'a>,
}

impl<'a> Global<'a> {
    pub fn new(
        object_store: (&'a mut HashSet<Object>, &'a mut HashSet<Object>),
        event_store: (
            &'a mut HashSet<events::global::GlobalEvent>,
            &'a mut HashSet<events::object::ObjectEvent>,
        ),
    ) -> Global<'a> {
        Global {
            date: 1,
            object: ObjectStore {
                alive: object_store.0,
                removed: object_store.1,
            },
            event: EventStore {
                global: event_store.0,
                object: event_store.1,
            },
        }
    }
}
