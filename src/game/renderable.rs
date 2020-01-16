use piston_window::{Graphics, Context};
use piston_window::math::Matrix2d;
use nphysics2d::object::DefaultBodySet;

pub trait Renderable {
    fn render<G: Graphics>(&self, context: Context, transform: Matrix2d, graphics: &mut G, world: &DefaultBodySet<f64>) -> ();
}
