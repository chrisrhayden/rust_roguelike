use std::collections::HashMap;

use crate::components::{
    entity::Entities, repr::Repr, size::Size, stats::Stats,
};

#[derive(Default)]
pub struct ComponentStore {
    pub player: u32,
    entities: Entities,
    pub stats: HashMap<u32, Stats>,
    pub repr: HashMap<u32, Repr>,
    pub size: HashMap<u32, Size>,
}

impl ComponentStore {
    pub fn make_player(&mut self, x: i32, y: i32) {
        let id = self.entities.new_entity();

        self.player = id;

        self.repr.insert(id, Repr { repr: '@', x, y });
        self.size.insert(id, Size::Medium);
    }
}
