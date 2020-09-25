use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::events;
use crate::models::object::{Object, ObjectKey};
use crate::utils::Date;

pub struct Global {
    pub date: Arc<Date>,
}

/// # Global
/// ## Properties
///
/// - date
///                   
///
impl Global {
    pub fn new(date: Arc<Date>) -> Global {
        Global { date }
    }
}
