use nphysics2d::object::DefaultBodySet;
use piston_window::math::Matrix2d;
use graphics::{Context, Graphics};
use opengl_graphics::{Texture, ImageSize};

pub trait Renderable {
    fn render<G: Graphics<Texture = T>, T: ImageSize>(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut G,
        world: &DefaultBodySet<f64>,
    ) -> ();
}
