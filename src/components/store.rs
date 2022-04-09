use std::collections::HashMap;

use crate::components::{entity::Entity, repr::Repr, stats::Stats};

#[derive(Default)]
pub struct ComponentStore {
    pub entities: HashMap<u32, Entity>,
    pub stats: HashMap<u32, Stats>,
    pub repr: HashMap<u32, Repr>,
}
