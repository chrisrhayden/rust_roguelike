use crate::{components::store::ComponentStore, map::map_data::MapData};

pub fn move_player(
    offset_x: i32,
    offset_y: i32,
    map: &MapData,
    store: &mut ComponentStore,
) {
    let player_id = store.player;
    let player = store.repr.get(&player_id).unwrap();

    let new_x: i32 = player.x as i32 + offset_x;
    let new_y: i32 = player.y as i32 + offset_y;

    for (id, r) in store.repr.iter() {
        if *id == player_id {
            continue;
        }

        if new_x == r.x && new_y == r.y {
            return;
        }
    }

    let index = (new_x + (new_y * map.map_width as i32)) as usize;

    if map.tiles[index].wall {
        return;
    }

    let mut player = store.repr.get_mut(&player_id).unwrap();
    player.x = new_x;
    player.y = new_y;
}
