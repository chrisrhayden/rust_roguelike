use rust_roguelike::{game::Game, sdl::SDLData, sprites::Sprites};

fn main() {
    let ascii_path = "./assets/dejavu10x10_gs_tc.png";
    let texture_width = 10;
    let texture_height = 10;
    let texture_columns = 32;

    let window_width = 800;
    let window_height = 600;

    let sdl_data = SDLData::new(window_width, window_height);

    // TODO: rename to sprites
    let tiles = Sprites::new(texture_width, texture_height, texture_columns);

    let mut game =
        Game::new(ascii_path, window_width, window_height, sdl_data, tiles);

    game.run();
}
