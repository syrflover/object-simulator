use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use crate::events;
use crate::models::object::{Object, ObjectKey};
use crate::utils::Date;

pub struct ObjectStore<'a> {
    pub alive: &'a mut HashMap<ObjectKey, Rc<Object>>,
    pub lifetime: &'a mut HashMap<ObjectKey, HashSet<Rc<Object>>>,
    pub removed: &'a mut HashMap<ObjectKey, Object>,
}

pub struct EventStore<'a> {
    pub global: &'a mut HashSet<events::global::GlobalEvent>,
    pub object: &'a mut HashSet<events::object::ObjectEvent>,
}

pub struct Global<'a> {
    pub date: Date,

    pub object: ObjectStore<'a>,

    pub event: EventStore<'a>,
}

/// # Global
/// ## Properties
///
/// - date
///                   
///
impl<'a> Global<'a> {
    pub fn new(
        object_store: (
            &'a mut HashMap<ObjectKey, Rc<Object>>,
            &'a mut HashMap<ObjectKey, HashSet<Rc<Object>>>,
            &'a mut HashMap<ObjectKey, Object>,
        ),
        event_store: (
            &'a mut HashSet<events::global::GlobalEvent>,
            &'a mut HashSet<events::object::ObjectEvent>,
        ),
    ) -> Global<'a> {
        Global {
            date: Date::new(0),
            object: ObjectStore {
                alive: object_store.0,
                lifetime: object_store.1,
                removed: object_store.2,
            },
            event: EventStore {
                global: event_store.0,
                object: event_store.1,
            },
        }
    }
}
