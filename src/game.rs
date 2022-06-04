use sdl2::{image::LoadTexture, pixels::Color, rect::Rect, render::Texture};

use crate::{
    components::store::ComponentStore,
    fov::shadow_casting::shadow_casting,
    map::map_data::{MapData, MapType, ViewTile},
    sdl::SDLData,
    sprites::Sprites,
    system::player::{handle_evt, move_player},
};

#[allow(dead_code)]
pub enum Action {
    Move(i32, i32),
    OpenMenu,
    Quit,
    Nothing,
}

enum GameLoopState {
    PlayerRun,
    WorldRun,
}

pub struct Game {
    window_width: u32,
    window_height: u32,
    state: GameLoopState,
    sprites: Sprites,
    sdl_data: SDLData,
    texture_path: String,
}

impl Game {
    pub fn new(
        texture_path: &str,
        window_width: u32,
        window_height: u32,
        sdl_data: SDLData,
        characters: Sprites,
    ) -> Self {
        Game {
            window_width,
            window_height,
            sdl_data,
            sprites: characters,
            state: GameLoopState::PlayerRun,
            texture_path: texture_path.into(),
        }
    }

    pub fn run(&mut self) {
        // its a pain to manage the texture in its own struct and there is only
        // one texture so ill just have it instantiated here
        let _img_context =
            sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
        let texture_creator = self.sdl_data.canvas.texture_creator();
        let mut texture = texture_creator
            .load_texture(&self.texture_path)
            .expect("cant load path");

        let map_width = self.window_width / self.sprites.width;
        let map_height = self.window_height / self.sprites.height;

        let mut store = ComponentStore::default();

        let mut map =
            MapData::new(&mut store, map_width, map_height, MapType::Walls);

        let mut view_map: Vec<ViewTile> = Vec::with_capacity(map.tiles.len());

        for _ in 0..map.tiles.len() {
            view_map.push(ViewTile {
                blocked: false,
                visible: false,
            });
        }

        let mut evt_pump = self
            .sdl_data
            .ctx
            .event_pump()
            .expect("cant build event pump");

        self.sdl_data.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.sdl_data.canvas.clear();

        self.sdl_data.canvas.present();

        loop {
            self.state = match self.state {
                GameLoopState::PlayerRun => match handle_evt(&mut evt_pump) {
                    Action::Move(x, y) => {
                        move_player(x, y, &map, &mut store);
                        GameLoopState::WorldRun
                    }
                    Action::Nothing => GameLoopState::PlayerRun,
                    Action::Quit => break,
                    _ => GameLoopState::PlayerRun,
                },
                GameLoopState::WorldRun => GameLoopState::PlayerRun,
            };

            self.sdl_data.canvas.clear();

            self.tick(&store, &map, &mut view_map);

            self.paint_map(&mut texture, &store, &mut map, &view_map);

            self.sdl_data.canvas.present();

            ::std::thread::sleep(::std::time::Duration::new(
                0,
                1_000_000_000u32 / 60,
            ));
        }
    }

    fn tick(
        &mut self,
        store: &ComponentStore,
        map: &MapData,
        view_map: &mut [ViewTile],
    ) {
        for (i, t) in map.tiles.iter().enumerate() {
            if t.wall {
                view_map[i].blocked = true;
                view_map[i].visible = false;
            } else {
                view_map[i].blocked = false;
                view_map[i].visible = false;
            }
        }

        for r in store.repr.values() {
            let index = r.x + (r.y * map.map_width as i32);

            view_map[index as usize].blocked = true;
        }

        let player = store.repr.get(&store.get_player()).unwrap();

        shadow_casting(
            view_map,
            map.map_width,
            map.map_height,
            player.x,
            player.y,
            5,
        );
    }

    fn paint_map(
        &mut self,
        texture: &mut Texture,
        store: &ComponentStore,
        map: &mut MapData,
        view_map: &[ViewTile],
    ) {
        let char_w = self.sprites.width;
        let char_h = self.sprites.height;

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut current_column = 1;
        for (i, t) in map.tiles.iter_mut().enumerate() {
            let brush = Rect::new(x, y, char_w, char_h);

            let c = if t.wall { '#' } else { '.' };

            let (to_paint, color) = if view_map[i].visible {
                t.visited = true;
                (self.sprites.get_rect(c), (0, 190, 190))
            } else if t.visited {
                t.visited = true;
                (self.sprites.get_rect(c), (93, 93, 93))
            } else {
                (self.sprites.get_rect(' '), (0, 0, 0))
            };

            texture.set_color_mod(color.0, color.1, color.2);
            self.sdl_data
                .canvas
                .copy(texture, Some(to_paint), Some(brush))
                .expect("cant copy to canvas");

            if current_column == map.map_width {
                x = 0;
                y += char_h as i32;
                current_column = 1;
            } else {
                current_column += 1;
                x += char_w as i32;
            }
        }

        let player_id = store.get_player();

        for (id, repr) in store.repr.iter() {
            let index = (repr.x + (repr.y * map.map_width as i32)) as usize;

            if !view_map[index].visible && *id != player_id {
                continue;
            }

            let to_paint = self.sprites.get_rect(repr.repr);

            let x = repr.x * char_w as i32;
            let y = repr.y * char_h as i32;

            let brush = Rect::new(x, y, char_w, char_h);

            texture.set_color_mod(0, 190, 190);
            self.sdl_data
                .canvas
                .copy(texture, Some(to_paint), Some(brush))
                .expect("cant copy to canvas");
        }
    }
}
