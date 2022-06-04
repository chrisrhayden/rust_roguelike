use sdl2::{event::Event, keyboard::Keycode, EventPump};

use crate::{
    components::store::ComponentStore, game::Action, map::map_data::MapData,
};

pub fn handle_evt(evt_pump: &mut EventPump) -> Action {
    for event in evt_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => return Action::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => return Action::Move(-1, 0),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => return Action::Move(1, 0),
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => return Action::Move(0, -1),
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => return Action::Move(0, 1),
            _ => {}
        }
    }

    Action::Nothing
}

pub fn move_player(
    offset_x: i32,
    offset_y: i32,
    map: &MapData,
    store: &mut ComponentStore,
) {
    let player_id = store.get_player();
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
