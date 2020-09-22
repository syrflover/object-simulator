use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(fmt::Debug)]
pub enum ObjectKey {
    Id(u64),
    Lifetime(u64),
}

impl PartialEq for ObjectKey {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ObjectKey::Id(self_value) => match other {
                ObjectKey::Id(other_value) => self_value == other_value,
                _ => false,
            },

            ObjectKey::Lifetime(self_lifetime) => match other {
                ObjectKey::Lifetime(other_lifetime) => self_lifetime == other_lifetime,
                _ => false,
            },
        }
    }
}

impl Eq for ObjectKey {}

impl Hash for ObjectKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ObjectKey::Id(id) => {
                id.hash(state);
            }
            ObjectKey::Lifetime(lifetime) => {
                lifetime.hash(state);
            }
        }
    }
}

pub struct Object {
    pub id: u64,
    pub alive: bool,
    pub lifetime: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Object")
            .field("id", &self.id)
            .field("alive", &self.alive)
            .field("lifetime", &self.lifetime)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
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
    pub fn new(id: u64, lifetime: u64, now_date: u64) -> Object {
        Object {
            id,
            alive: true,
            lifetime: lifetime + now_date,
            created_at: now_date,
            updated_at: now_date,
        }
    }

    pub fn generate_fake_object(id: u64) -> Object {
        Object {
            id,
            alive: true,
            lifetime: 0,
            created_at: 0,
            updated_at: 0,
        }
    }

    pub fn remove(&self) -> Object {
        Object {
            id: self.id,
            alive: false,
            lifetime: self.lifetime,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
