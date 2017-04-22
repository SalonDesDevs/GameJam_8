use super::super::app::*;
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::*;

pub struct Menu;

impl State for Menu {

    fn create(&mut self, app: &mut App) {
        
    }

    fn render(&mut self, app: &mut App) {
        let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();
        let transform = app.context.transform.trans(10.0, 100.0);

        clear([0.0, 0.0, 0.0, 1.0], app.graphics);
        text::Text::new_color([0.0, 1.0, 1.0, 1.0], 32)
            .draw("Menu ", &mut glyphs, &(app.context.draw_state), transform, app.graphics);
 
    }

    fn close(&mut self, app: &mut App) {
    
    }

}
