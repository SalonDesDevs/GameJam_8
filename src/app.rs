use graphics::draw_state::DrawState;
use opengl_graphics::GlGraphics;
use graphics::context::Context;
use graphics::*;
use opengl_graphics::glyph_cache::GlyphCache;

pub struct App<'a> {
    graphics: & 'a mut GlGraphics,
    context: Context
}

pub struct AppBuilder<'a> {
    graphics: & 'a mut GlGraphics,
    context: Context
}

impl<'a> App<'a> {

    pub fn render(&mut self) {
        let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();
        let transform = self.context.transform.trans(10.0, 100.0);

        clear([0.0, 0.0, 0.0, 1.0], self.graphics);
        text::Text::new_color([0.0, 1.0, 1.0, 1.0], 32)
            .draw("Hello world ! ", &mut glyphs, &(self.context.draw_state), transform, self.graphics);
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
            context: self.context
        } 
    }

}
