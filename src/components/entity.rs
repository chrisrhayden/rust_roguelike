pub struct Entity(u32);

#[derive(Default)]
pub struct Entities {
    last_id: u32,
}

impl Entities {
    pub fn new_entity(&mut self) -> Entity {
        self.last_id += 1;

        Entity(self.last_id)
    }
}
