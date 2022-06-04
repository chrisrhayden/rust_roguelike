use crate::components::stats::Stats;

pub enum Enemy {
    Goblin,
    Troll,
}

impl Enemy {
    pub fn gen_stats(&self) -> Stats {
        match self {
            Enemy::Troll => Stats {
                base_health: 10,
                base_power: 10,
            },
            Enemy::Goblin => Stats {
                base_health: 5,
                base_power: 5,
            },
        }
    }
}
