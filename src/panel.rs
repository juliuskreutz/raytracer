use sdl2::rect::Point;
use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

pub struct Panel {
    canvas: Canvas<Window>,
}

impl Panel {
    pub const WIDTH: u32 = 640;
    pub const HEIGHT: u32 = 480;

    pub fn new(title: &str, sdl: Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window(title, Self::WIDTH, Self::HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self { canvas }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(Point::new(x, y)).unwrap()
    }

    pub fn present(&mut self) {
        self.canvas.present()
    }
}
