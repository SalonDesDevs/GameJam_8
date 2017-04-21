extern crate piston_window;
extern crate graphics;
extern crate conrod;

use piston_window::*;

const VERSION: & 'static str = "0.1";
const NAME: & 'static str = "History Step";
const SIZE: [u32; 2] = [600, 600];
const FULLSCREEN: bool = false;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new(format!("{} - {}", NAME, VERSION), SIZE)
        .exit_on_esc(true).fullscreen(FULLSCREEN).build().unwrap();
}
