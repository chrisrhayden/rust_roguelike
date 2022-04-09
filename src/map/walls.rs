use std::cmp::{max, min};

use rand::{prelude::*, thread_rng, Rng};

use crate::{components::store::ComponentStore, map::map_data::Tile};

#[derive(Debug)]
struct Room {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Room {
    fn new(x1: u32, y1: u32, width: u32, height: u32) -> Self {
        Self {
            x1,
            y1,
            x2: x1 + width,
            y2: y1 + height,
        }
    }

    fn intersects(&self, other: &Room) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }

    fn center(&self) -> (u32, u32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub struct Walls {
    max_rooms: u32,
    max_room_size: u32,
    min_room_size: u32,
    map_width: u32,
    map_height: u32,
    map: Vec<Tile>,
}

impl Walls {
    pub fn new(map_width: u32, map_height: u32) -> Self {
        let size: usize = map_width as usize * map_height as usize;

        let map: Vec<Tile> = vec![Tile::Wall; size];

        Self {
            max_rooms: 30,
            max_room_size: 8,
            min_room_size: 4,
            map_width,
            map_height,
            map,
        }
    }

    pub fn gen(mut self, store: &mut ComponentStore) -> Vec<Tile> {
        let mut rng = thread_rng();

        let mut rooms: Vec<Room> = Vec::new();

        'make_rooms: for _ in 0..self.max_rooms {
            let room_width =
                rng.gen_range(self.min_room_size..=self.max_room_size);
            let room_height =
                rng.gen_range(self.min_room_size..=self.max_room_size);

            let x = rng.gen_range(1..(self.map_width - room_width - 1));
            let y = rng.gen_range(1..(self.map_height - room_height - 1));

            println!("r {} {} {} {}", x, y, room_width, room_height);

            let room = Room::new(x, y, room_width, room_height);

            for r in &rooms {
                if r.intersects(&room) {
                    continue 'make_rooms;
                }
            }

            self.carve_out_room(&room);

            if let Some(last_room) = rooms.last() {
                self.carve_out_hallway(&mut rng, last_room, &room);
            } else {
                let (player_x, player_y) = room.center();

                println!("p {} {} ", player_x, player_y);
                store.make_player(player_x, player_y);
            }

            rooms.push(room);
        }

        self.map
    }

    fn carve_out_room(&mut self, room: &Room) {
        for y in room.y1..=room.y2 {
            for x in room.x1..=room.x2 {
                let index = x + (y * self.map_width);

                self.map[index as usize] = Tile::Floor;
            }
        }
    }

    fn carve_out_hallway(
        &mut self,
        rng: &mut ThreadRng,
        past_room: &Room,
        room: &Room,
    ) {
        let (c_x, c_y) = room.center();
        let (p_x, p_y) = past_room.center();

        // start from either the past room or current room
        let (sx, sy) = if rng.gen_bool(0.5) {
            (c_x, p_y)
        } else {
            (p_x, c_y)
        };

        let min_x = min(p_x, c_x);
        let max_x = max(p_x, c_x);

        for x in min_x..=max_x {
            let index = x + (sy * self.map_width);

            self.map[index as usize] = Tile::Floor;
        }

        let min_y = min(p_y, c_y);
        let max_y = max(p_y, c_y);

        for y in min_y..=max_y {
            let index = sx + (y * self.map_width);

            self.map[index as usize] = Tile::Floor;
        }
    }
}
