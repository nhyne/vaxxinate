use graphics::Context;
use nphysics2d::object::DefaultBodySet;
use opengl_graphics::GlGraphics;
use piston_window::math::Matrix2d;

pub trait Renderable {
    fn render(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut GlGraphics,
        world: &DefaultBodySet<f64>,
    ) -> ();
}
