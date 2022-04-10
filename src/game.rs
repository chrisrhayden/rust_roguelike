use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color,
    rect::Rect, render::Texture, EventPump,
};

use crate::{
    characters::Characters,
    components::store::ComponentStore,
    map::map_data::{MapData, MapType, Tile, ViewTile},
    sdl::SDLData,
};

#[allow(dead_code)]
enum Action {
    Move,
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
    characters: Characters,
    sdl_data: SDLData,
    texture_path: String,
}

impl Game {
    pub fn new(
        texture_path: &str,
        window_width: u32,
        window_height: u32,
        sdl_data: SDLData,
        characters: Characters,
    ) -> Self {
        Game {
            window_width,
            window_height,
            sdl_data,
            characters,
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
        let texture = texture_creator
            .load_texture(&self.texture_path)
            .expect("cant load path");

        let map_width = self.window_width / self.characters.width;
        let map_height = self.window_height / self.characters.height;

        let mut store = ComponentStore::default();
        let map =
            MapData::new(&mut store, map_width, map_height, MapType::Walls);

        let mut evt_pump = self
            .sdl_data
            .ctx
            .event_pump()
            .expect("cant build event pump");

        self.sdl_data.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.sdl_data.canvas.clear();

        self.sdl_data.canvas.present();

        loop {
            self.state = match self.state {
                GameLoopState::PlayerRun => {
                    match self.handle_evt(&mut evt_pump) {
                        Action::Move => GameLoopState::WorldRun,
                        Action::Nothing => GameLoopState::PlayerRun,
                        Action::Quit => break,
                        _ => GameLoopState::PlayerRun,
                    }
                }
                GameLoopState::WorldRun => GameLoopState::PlayerRun,
            };

            self.sdl_data.canvas.clear();

            self.tick(&map);
            self.paint_map(&texture, &store, &map);

            self.sdl_data.canvas.present();

            ::std::thread::sleep(::std::time::Duration::new(
                0,
                1_000_000_000u32 / 60,
            ));
        }
    }

    fn handle_evt(&self, evt_pump: &mut EventPump) -> Action {
        for event in evt_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Action::Quit,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => return Action::Move,
                _ => {}
            }
        }

        Action::Nothing
    }

    fn tick(&mut self, map: &MapData) {
        let mut view_map = Vec::with_capacity(map.tiles.len());

        for t in &map.tiles {
            if *t == Tile::Wall {
                view_map.push(ViewTile {
                    blocked: true,
                    visible: false,
                });
            } else {
                view_map.push(ViewTile {
                    blocked: false,
                    visible: false,
                });
            }
        }
    }

    fn paint_map(
        &mut self,
        texture: &Texture,
        store: &ComponentStore,
        map: &MapData,
    ) {
        let char_w = self.characters.width;
        let char_h = self.characters.height;

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut current_column = 1;
        for t in &map.tiles {
            let brush = Rect::new(x, y, char_w, char_h);

            let c = match *t {
                Tile::Wall => '#',
                Tile::Floor => ' ',
            };

            let to_paint = self.characters.get_rect(c);

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

        for (_id, repr) in store.repr.iter() {
            let to_paint = self.characters.get_rect(repr.repr);

            let x = (repr.x * char_w) as i32;
            let y = (repr.y * char_h) as i32;

            let brush = Rect::new(x, y, char_w, char_h);

            self.sdl_data
                .canvas
                .copy(texture, Some(to_paint), Some(brush))
                .expect("cant copy to canvas");
        }
    }
}
