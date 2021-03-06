use crate::utils::Date;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

use crate::models;

pub enum ObjectEvent {
    Nope,
    Birth(f64),
    Death(f64),
}
/* {
    pub birth: Box<dyn Fn(u64, u64) -> models::object::Object>,

    pub death: Box<
        dyn Fn(
            &models::object::Object,
            &mut HashSet<models::object::Object>,
            &mut HashSet<models::object::Object>,
        ) -> (),
    >,
} */

impl ObjectEvent {
    pub fn new() -> ObjectEvent {
        ObjectEvent::Nope
    }

    pub fn generate_percentage_table() -> Vec<ObjectEvent> {
        vec![ObjectEvent::Birth(0.012), ObjectEvent::Death(0.01)]
    }

    pub fn percentage(&self) -> Option<f64> {
        match self {
            Self::Birth(percentage) => Some(*percentage),
            Self::Death(percentage) => Some(*percentage),
            _ => None,
        }
    }

    pub fn birth(
        &self,
        object_store: Arc<Mutex<models::object::ObjectStore>>,
        date: Arc<Date>,
        id: u64,
    ) -> () {
        let new_object = models::object::Object::new(id, date.to_day());

        println!("Object = {:?}", &new_object);

        object_store
            .lock()
            .unwrap()
            .insert(id, Arc::new(new_object));
    }

    pub fn death(&self, object_store: Arc<Mutex<models::object::ObjectStore>>, id: u64) {
        let mut object_store = object_store.lock().unwrap();

        if let Some(object) = object_store.get(&id) {
            let object = object.remove();

            object_store.insert(id, Arc::new(object));
        }
    }
}

/* pub enum ObjectEventType {
    Birth {
        runner: Box<dyn Fn(u64, u64) -> models::object::Object>,
    },
    Death {
        runner: Box<dyn Fn(&models::object::Object) -> models::object::Object>,
    }, // Death(&'a mut Object),
}

impl PartialEq for ObjectEventType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct ObjectEvent {
    pub id: u32,
    pub percentage: f64,
    pub event_type: ObjectEventType,
}

impl PartialEq for ObjectEvent {
    fn eq(&self, other: &Self) -> bool {
        self.event_type == other.event_type
    }
}
impl Eq for ObjectEvent {}
impl Hash for ObjectEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ObjectEvent {
    pub fn new(id: u32, percentage: f64, event_type: ObjectEventType) -> ObjectEvent {
        ObjectEvent {
            id,
            percentage,
            event_type,
        }
    }
}

pub fn generate_event_table() -> HashSet<ObjectEvent> {
    let mut event_table = HashSet::new();

    let birth_event_runner = Box::new(|id, date| {
        let new_object = models::object::Object::new(id, date);
        // println!("Occurs Event :: Created {:?}", new_object);

        new_object
    });
    let birth_event = ObjectEvent::new(
        1,
        0.05,
        ObjectEventType::Birth {
            runner: birth_event_runner,
        },
    );

    let death_event_runner: Box<dyn Fn(&models::object::Object) -> models::object::Object> =
        Box::new(|object| {
            let removed_object = object.remove();

            // println!("Occurs Event :: Removed {:?}", removed_object);

            removed_object
        });
    let death_event = ObjectEvent::new(
        2,
        0.028,
        ObjectEventType::Death {
            runner: death_event_runner,
        },
    );

    event_table.insert(birth_event);
    event_table.insert(death_event);

    event_table
} */
