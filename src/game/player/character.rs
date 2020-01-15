use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{DefaultBodyHandle, DefaultBodySet, RigidBodyDesc};

const PLAYER_BODY_WIDTH: f64 = 20.0;
const PLAYER_BODY_HEIGHT: f64 = 20.0;

pub struct Character {
    name: &'static str,
    handle: DefaultBodyHandle,
}

impl Character {
    pub fn new(body_set: &mut DefaultBodySet<f64>) -> Character {
        let character_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            PLAYER_BODY_WIDTH,
            PLAYER_BODY_HEIGHT,
        )));

        let character_body = RigidBodyDesc::new()
            .position(Isometry2::translation(10.0, 10.1))
            .build();

        let handle = body_set.insert(character_body);
        Character {
            name: "something",
            handle,
        }
    }
}
