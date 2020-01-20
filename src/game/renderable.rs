use nphysics2d::object::DefaultBodySet;
use piston_window::math::Matrix2d;
//use piston_window::{Context, Graphics};
use graphics::{Context, Graphics, ImageSize};
use opengl_graphics::Texture;

pub trait Renderable {
    fn render<G: Graphics<Texture = T>, T: ImageSize>(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut G,
        world: &DefaultBodySet<f64>,
    ) -> ();
}
