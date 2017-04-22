use graphics::draw_state::DrawState;
use opengl_graphics::GlGraphics;
use graphics::context::Context;
use graphics::*;
use states::*;

pub struct App<'a> {
    pub graphics: & 'a mut GlGraphics,
    pub context: Context,
    pub state: Box<State>
}

pub struct AppBuilder<'a> {
    graphics: & 'a mut GlGraphics,
    context: Context
}

pub trait State {
    
    fn create(&mut self, app: &mut App) {}

    fn render(&mut self, app: &mut App) {}

    fn close(&mut self, app: &mut App) {}
}

impl<'a> App<'a> {
    pub fn render(&self) {
       self.state.render(&self);
    }
}

impl<'a> AppBuilder<'a> {

    pub fn new(g: &mut GlGraphics, c: Context) -> AppBuilder {
        AppBuilder {
            graphics: g,
            context: c
        }
    }

    pub fn build(self) -> App<'a> {
        App { 
            graphics: self.graphics,
            context: self.context,
            state: Box::new(loading::Loading)
        } 
    }

}
