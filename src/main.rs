extern crate object_simulator;

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use tokio::prelude::*;

use rand::prelude::*;

use crate::object_simulator::events;
use crate::object_simulator::models;
use crate::object_simulator::utils::{Date, YEAR};

#[tokio::main]
async fn main() {
    //
    //  Initialize Global
    //
    let mut global = models::global::Global::new(Arc::new(Date::new(1)));
    let object_store: Arc<Mutex<models::object::ObjectStore>> =
        Arc::new(Mutex::new(HashMap::new()));
    // let mut global_event_store = HashSet::new();

    //
    // Event

    //
    let object_event = Arc::new(events::object::ObjectEvent::new());
    let object_event_table = Arc::new(events::object::ObjectEvent::generate_percentage_table());

    //
    // Initialize Object
    //
    object_event.birth(Arc::clone(&object_store), Arc::clone(&global.date), 1);

    //

    loop {
        let object_len = object_store.lock().unwrap().len();
        // .iter()
        // .filter(|(_, object)| object.alive)
        // .count();

        'object_iteration: for id in 1..object_len + 1 {
            let object_store = Arc::clone(&object_store);
            let object_event = Arc::clone(&object_event);
            let object_event_table = Arc::clone(&object_event_table);
            let date = Arc::clone(&global.date);

            tokio::spawn(async move {
                let mut rng = rand::thread_rng();

                let id: u64 = id.try_into().unwrap();

                'event_iteration: for event in object_event_table.iter() {
                    let rand_num: f64 = rng.gen();

                    if rand_num <= event.percentage().unwrap() {
                        // 생존 여부 확인 후
                        // 죽었으면 continue

                        if let Some(object) = object_store.lock().unwrap().get(&id) {
                            println!("obje = {:?}", &object);
                            if object.alive == false {
                                break 'event_iteration;
                            }
                        }

                        match event {
                            events::object::ObjectEvent::Birth(_) => {
                                let new_object_id: u64 =
                                    (object_store.lock().unwrap().len() + 1).try_into().unwrap();

                                object_event.birth(
                                    Arc::clone(&object_store),
                                    Arc::clone(&date),
                                    new_object_id,
                                );
                            }

                            events::object::ObjectEvent::Death(_) => {
                                object_event.death(Arc::clone(&object_store), id);
                            }

                            _ => {}
                        }
                    }
                }
            });
        }

        let object_store = object_store.lock().unwrap();

        let object_len = object_store.len();
        let alive_len = object_store
            .iter()
            .filter(|(_, object)| object.alive)
            .count();
        let removed_len = object_len - alive_len;

        if alive_len == 0 {
            panic!("The End");
        }

        println!("date = {}", &global.date);
        println!("all = {}", object_len);
        println!("alive = {}", alive_len);
        println!("removed = {}", removed_len);

        global.date = Arc::new(Date::new(global.date.to_day() + 1));

        let duration = time::Duration::from_millis(105);
        thread::sleep(duration);
    }
}

async fn process_object_event(object_event: events::object::ObjectEvent) {}
