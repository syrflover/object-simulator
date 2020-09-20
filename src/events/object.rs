use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::models;

pub enum ObjectEventType {
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
        println!("Occurs Event :: Created {:?}", new_object);

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

            println!("Occurs Event :: Removed {:?}", removed_object);

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
}
