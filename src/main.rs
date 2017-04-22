extern crate graphics;
extern crate conrod;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::*;
use opengl_graphics::glyph_cache::GlyphCache;

const VERSION: & 'static str = "0.1";
const NAME: & 'static str = "History Step";
const SIZE: [u32; 2] = [600, 600];
const FULLSCREEN: bool = false;

mod app;

fn main() {
    let opengl = OpenGL::V3_2;
    /// Create the window instance
    let mut window: Sdl2Window = Sdl2Window::new(
        &WindowSettings::new(format!("{} - {}", NAME, VERSION) , SIZE)
        .fullscreen(FULLSCREEN)
        .vsync(true)
        .opengl(opengl)
    ).unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let mut app = app::AppBuilder::new(g, c).build();
                app.render();                
            });
        }
    }

}
