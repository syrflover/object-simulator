extern crate object_simulator;

use std::collections::HashSet;
use std::convert::TryInto;
use std::{thread, time};

use rand::prelude::*;

use crate::object_simulator::events;
use crate::object_simulator::models;

fn main() -> Result<(), std::num::TryFromIntError> {
    let mut alive_object_store = HashSet::new();
    let mut removed_object_store = HashSet::new();
    let mut object_event_store = HashSet::new();
    let mut global_event_store = HashSet::new();

    let mut global = models::global::Global::new(
        (&mut alive_object_store, &mut removed_object_store),
        (&mut global_event_store, &mut object_event_store),
    );

    let object_event_table = events::object::generate_event_table();

    global
        .object
        .alive
        .insert(models::object::Object::new(1, 1));

    let mut rng = rand::thread_rng();

    loop {
        for object_event in object_event_table.iter() {
            for object_id in 1..global.object.alive.len() + 1 {
                let rand_num: f64 = rng.gen();

                if rand_num <= object_event.percentage {
                    match &object_event.event_type {
                        events::object::ObjectEventType::Birth { runner } => {
                            let id = global.object.alive.len() + global.object.removed.len() + 1;

                            let new_object = (runner)(id.try_into().unwrap(), global.date);

                            global.object.alive.insert(new_object);
                        }

                        events::object::ObjectEventType::Death { runner } => {
                            let fake_object = generate_fake_object(object_id.try_into().unwrap());

                            if let Some(object) = global.object.alive.get(&fake_object) {
                                let removed_object = (runner)(&object);

                                global.object.alive.remove(&removed_object);
                                global.object.removed.insert(removed_object);

                                // println!("Removed {:?}", &removed_object);
                            }
                        }
                    }
                }
            }
        }

        let duration = time::Duration::from_millis(100);
        thread::sleep(duration);

        let alive_object_count = &global.object.alive.len();
        let removed_object_count = &global.object.removed.len();

        println!("-------------------------------");
        println!("Date            = {}", &global.date);
        println!(
            "All Objects     = {}",
            alive_object_count + removed_object_count
        );
        println!("Alive Objects   = {}", &alive_object_count);
        println!("Removed Objects = {}", &removed_object_count);
        println!("-------------------------------");

        if alive_object_count == &0 {
            panic!("The End");
        }

        global.date += 1;
    }
}

fn generate_fake_object(id: u64) -> models::object::Object {
    models::object::Object {
        id,
        alive: true,
        created_at: 0,
        updated_at: 0,
    }
}
