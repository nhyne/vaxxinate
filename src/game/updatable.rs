use nphysics2d::object::DefaultBodySet;
use opengl_graphics::Texture;
use piston_window::Key;
use sprite::Scene;
use std::collections::HashSet;

trait MutBodySetWithInputUpdatable {
    fn update(
        body_set: &mut DefaultBodySet<f64>,
        keys_pressed: HashSet<Key>,
        scene: &mut Scene<Texture>,
    ) -> ();
}

trait BodySetUpdatable {
    fn update(body_set: &DefaultBodySet<f64>, scene: &mut Scene<Texture>) -> ();
}
