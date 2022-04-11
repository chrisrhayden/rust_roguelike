use crate::{components::store::ComponentStore, map::walls::Walls};

#[derive(Clone, PartialEq, Eq)]
pub struct Tile {
    pub wall: bool,
    pub visited: bool,
}

pub struct ViewTile {
    pub blocked: bool,
    pub visible: bool,
}

#[derive(Debug)]
pub enum MapType {
    Walls,
    CellularAutomate,
}

pub struct MapData {
    pub map_width: u32,
    pub map_height: u32,
    pub tiles: Vec<Tile>,
}

impl MapData {
    pub fn new(
        store: &mut ComponentStore,
        map_width: u32,
        map_height: u32,
        map_type: MapType,
    ) -> Self {
        let tiles = match map_type {
            MapType::Walls => {
                let wall_data = Walls::new(map_width, map_height);

                wall_data.gen(store)
            }
            _ => {
                panic!("{:?} is not implemented", map_type);
            }
        };

        Self {
            map_width,
            map_height,
            tiles,
        }
    }
}
