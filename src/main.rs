extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod game_model;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {}
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App { gl: GlGraphics::new(opengl) };

    let mut events = Events::new(EventSettings::new());

    //let mut game_model = game_model::Arena::new(50);

    loop {
        let nextFrame: Option<Input> = events.next(&mut window);

        match nextFrame {
            Some(frame) => println!("test"),
            None => println!("Dunno what circumstances this would happen under... :P"),
        }
    }
}
