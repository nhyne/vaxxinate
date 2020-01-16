use nphysics2d::object::DefaultBodySet;
use piston_window::math::Matrix2d;
use piston_window::{Context, Graphics};

pub trait Renderable {
    fn render<G: Graphics>(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut G,
        world: &DefaultBodySet<f64>,
    ) -> ();
}
