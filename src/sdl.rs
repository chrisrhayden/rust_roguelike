use sdl2::{render::Canvas, video::Window, Sdl};

pub struct SDLData {
    pub ctx: Sdl,
    pub canvas: Canvas<Window>,
}

impl SDLData {
    pub fn new(width: u32, height: u32) -> Self {
        let ctx = sdl2::init().expect("cant initialize sdl");

        let vido_subsystem = ctx.video().expect("cant get video subsystem");

        let window = vido_subsystem
            .window("rust roguelike", width, height)
            .position_centered()
            .build()
            .expect("cant build window");

        let canvas = window.into_canvas().build().expect("cant get canvas");

        Self { ctx, canvas }
    }
}
