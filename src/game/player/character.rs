use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, RigidBodyDesc,
};

const PLAYER_BODY_WIDTH: f64 = 20.0;
const PLAYER_BODY_HEIGHT: f64 = 20.0;

pub struct Character {
    name: &'static str,
    // It is possible to get the body handle from the collider handle following the example below
    //      Assuming `collider_handle` is a valid handle of a collider previously added to the world.
    //      let collider = collider_set.get(collider_handle).expect("Collider not found.");
    //      let body_handle = collider.body();
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
}

impl Character {
    pub fn new(
        body_set: &mut DefaultBodySet<f64>,
        collider_set: &mut DefaultColliderSet<f64>,
    ) -> Character {
        let character_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            PLAYER_BODY_WIDTH,
            PLAYER_BODY_HEIGHT,
        )));

        let character_collider = ColliderDesc::new(character_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));

        let character_body = RigidBodyDesc::new()
            .position(Isometry2::translation(10.0, 10.1))
            .build();

        let body_handle = body_set.insert(character_body);

        let character_collider = character_collider.build(BodyPartHandle(body_handle, 0));
        let collider_handle = collider_set.insert(character_collider);

        Character {
            name: "something",
            body_handle,
            collider_handle,
        }
    }
}
