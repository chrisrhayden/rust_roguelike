use rand::{prelude::ThreadRng, Rng};

use crate::{
    components::{enemies::Enemy, repr::Repr, store::ComponentStore},
    map::map_data::MapRect,
};

pub struct FeatureGen {
    pub level: u32,
}

impl FeatureGen {
    pub fn gen_enemys(
        &self,
        rng: &mut ThreadRng,
        room: &MapRect,
        store: &mut ComponentStore,
    ) {
        let max_enemys: usize = match self.level {
            1 => 3,
            2 => 4,
            3 => 5,
            _ => panic!("higher levels not supported"),
        };

        let enemies: usize = rng.gen_range(1..=max_enemys);
        let mut made_ents = 0;

        let mut entity_pos: Vec<(i32, i32)> = vec![];

        'make_enemys: while made_ents < enemies {
            let x = rng.gen_range(room.x1..=room.x2) as i32;
            let y = rng.gen_range(room.y1..=room.y2) as i32;

            for (e_x, e_y) in &entity_pos {
                if x == *e_x && y == *e_y {
                    continue 'make_enemys;
                }
            }

            let ent_id = store.make_entity();

            let (stat, repr) = if rng.gen_bool(0.05) {
                (Enemy::Troll.gen_stats(), Repr { x, y, repr: 'T' })
            } else {
                (Enemy::Goblin.gen_stats(), Repr { x, y, repr: 'g' })
            };

            entity_pos.push((x, y));

            store.repr.insert(ent_id, repr);
            store.stats.insert(ent_id, stat);

            made_ents += 1;
        }
    }
}
