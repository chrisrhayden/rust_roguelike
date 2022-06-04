use crate::{
    components::store::ComponentStore, feature_gen::FeatureGen,
    map::walls::Walls,
};

#[derive(Debug)]
pub struct MapRect {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl MapRect {
    pub fn new(x1: u32, y1: u32, width: u32, height: u32) -> Self {
        Self {
            x1,
            y1,
            x2: x1 + width,
            y2: y1 + height,
        }
    }

    pub fn intersects(&self, other: &MapRect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }

    pub fn center(&self) -> (u32, u32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

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
                let feature_gen = FeatureGen { level: 1 };
                let wall_data = Walls::new(map_width, map_height);

                wall_data.gen(&feature_gen, store)
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
