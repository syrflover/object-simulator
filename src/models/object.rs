use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Object {
    pub id: u64,
    pub alive: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Object {}
impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Object {
    pub fn new(id: u64, now_date: u64) -> Object {
        Object {
            id,
            alive: true,
            created_at: now_date,
            updated_at: now_date,
        }
    }

    pub fn remove(&self) -> Object {
        Object {
            id: self.id,
            alive: false,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
