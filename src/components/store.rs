use std::collections::HashMap;

use crate::components::{entity::Entities, repr::Repr, stats::Stats};

#[derive(Default)]
pub struct ComponentStore {
    entities: Entities,
    pub stats: HashMap<u32, Stats>,
    pub repr: HashMap<u32, Repr>,
}

impl ComponentStore {
    pub fn make_player(&mut self, x: u32, y: u32) {
        let id = self.entities.new_entity();

        self.repr.insert(id, Repr { repr: '@', x, y });
    }
}
