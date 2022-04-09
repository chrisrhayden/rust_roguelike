#[derive(Default)]
pub struct Entities {
    last_id: u32,
}

impl Entities {
    pub fn new_entity(&mut self) -> u32 {
        self.last_id += 1;

        self.last_id
    }
}
