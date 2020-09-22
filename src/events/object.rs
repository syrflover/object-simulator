use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use crate::models;

pub struct ObjectEvent;
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
        ObjectEvent
    }

    pub fn generate_percentage_table() -> Vec<ObjectEventPercentage> {
        vec![
            ObjectEventPercentage::Birth(0.0005),
            // ObjectEventPercentage::Death(0.003),
        ]
    }

    pub fn birth(&self, global: &mut models::global::Global, id: u64, lifetime: u64) -> () {
        let new_object = models::object::Object::new(id, lifetime, global.date.to_day());
        let new_object = Rc::new(new_object);
        let new_object_lifetime = models::object::ObjectKey::Lifetime(new_object.lifetime);

        global
            .object
            .alive
            .insert(models::object::ObjectKey::Id(id), Rc::clone(&new_object));

        if let Some(object_hash_set) = global.object.lifetime.get_mut(&new_object_lifetime) {
            object_hash_set.insert(Rc::clone(&new_object));
        } else {
            let mut object_hash_set = HashSet::new();

            object_hash_set.insert(Rc::clone(&new_object));

            global
                .object
                .lifetime
                .insert(new_object_lifetime, object_hash_set);
        }
    }

    pub fn death(&self, global: &mut models::global::Global, id: u64) {
        let id = models::object::ObjectKey::Id(id);

        if let Some(object) = global.object.alive.get(&id) {
            let object = object.remove();

            let lifetime = models::object::ObjectKey::Lifetime(object.lifetime);

            global
                .object
                .lifetime
                .get_mut(&lifetime)
                .unwrap()
                .remove(&object);

            global.object.alive.remove(&id);
            // global.object.lifetime.remove(&lifetime);
            global.object.removed.insert(id, object);
        }
    }

    pub fn death_from_lifetime(
        &self,
        global: &mut models::global::Global,
        lifetime: &models::object::ObjectKey,
    ) {
        let alive_object_hash_map = &mut global.object.alive;
        let removed_object_hash_map = &mut global.object.removed;

        if let Some(object_hash_set) = global.object.lifetime.get(lifetime) {
            for object in object_hash_set.iter() {
                let object = object.remove();
                let id = models::object::ObjectKey::Id(object.id);

                alive_object_hash_map.remove(&id);
                removed_object_hash_map.insert(id, object);
            }

            global.object.lifetime.remove(lifetime);
        }
    }
}

pub enum ObjectEventPercentage {
    Birth(f64),
    Death(f64),
}

impl ObjectEventPercentage {
    pub fn percentage(&self) -> f64 {
        match self {
            ObjectEventPercentage::Birth(percentage) => *percentage,
            ObjectEventPercentage::Death(percentage) => *percentage,
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
