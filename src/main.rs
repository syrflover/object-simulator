extern crate object_simulator;

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::rc::Rc;
use std::{thread, time};

use rand::prelude::*;

use crate::object_simulator::events;
use crate::object_simulator::models;
use crate::object_simulator::utils::{Date, YEAR};

fn main() -> Result<(), std::num::TryFromIntError> {
    //
    //  Initialize Global
    //
    let mut alive_object_store = HashMap::new();
    let mut lifetime_object_store = HashMap::new();
    let mut removed_object_store = HashMap::new();
    let mut object_event_store = HashSet::new();
    let mut global_event_store = HashSet::new();

    let mut global = models::global::Global::new(
        (
            &mut alive_object_store,
            &mut lifetime_object_store,
            &mut removed_object_store,
        ),
        (&mut global_event_store, &mut object_event_store),
    );

    //
    // Event
    //
    let object_event_table = events::object::ObjectEvent::new();
    let object_event_percentage_table = events::object::ObjectEvent::generate_percentage_table();

    //
    // Initialize Object
    //
    object_event_table.birth(&mut global, 1, YEAR * 30);

    //
    let mut rng = rand::thread_rng();

    loop {
        let days_count = global.date.to_day();

        global.date = Date::new(days_count + 1);

        let alive_object_count = global.object.alive.len();
        let removed_object_count = global.object.removed.len();
        let all_object_count = alive_object_count + removed_object_count;

        let object_lifetime = models::object::ObjectKey::Lifetime(days_count);

        if global.date.month == 1 && global.date.day == 1 {
            let duration = time::Duration::from_millis(100);
            thread::sleep(duration);

            println!("Date            = {}", global.date);
            println!("All Objects     = {}", alive_object_count);
            println!("Alive Objects   = {}", alive_object_count);
            println!("Removed Objects = {}", removed_object_count);
            println!("-------------------------------");
            println!("Days {}", days_count);
            println!("Lifetime len {}", global.object.lifetime.len());
            println!("Lifetimes {:?}", &object_lifetime);

            if alive_object_count == 0 {
                panic!("The End");
            }
        }

        object_event_table.death_from_lifetime(&mut global, &object_lifetime);

        'object_iteration: for object_id in 1..(all_object_count) + 1 {
            let object_id: u64 = object_id.try_into().unwrap();

            if !global
                .object
                .alive
                .contains_key(&models::object::ObjectKey::Id(object_id))
            {
                continue 'object_iteration;
            }

            let rand_num: f64 = rng.gen();

            for event_percentage in &object_event_percentage_table {
                let percentage = event_percentage.percentage();

                if rand_num <= percentage {
                    match event_percentage {
                        events::object::ObjectEventPercentage::Birth(_) => {
                            let new_object_id: u64 = (all_object_count + 1).try_into().unwrap();

                            object_event_table.birth(&mut global, new_object_id, YEAR * 30);
                        }

                        _ => {} /* events::object::ObjectEventPercentage::Death(_) => {
                                    // object_event_table.death(&mut global, object_id);
                                } */
                    }
                }
            }
        }
    }
}
