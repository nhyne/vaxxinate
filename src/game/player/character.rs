use crate::game::renderable::Renderable;

use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, RigidBodyDesc,
};
use piston_window::math::Matrix2d;
use piston_window::{Context, Graphics, Rectangle};
use std::borrow::Borrow;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CHARACTER_BODY_WIDTH: f64 = 20.0;
const CHARACTER_BODY_HEIGHT: f64 = 20.0;
const CHARACTER_RENDER_WIDTH: f64 = CHARACTER_BODY_WIDTH * 2.0;
const CHARACTER_RENDER_HEIGHT: f64 = CHARACTER_BODY_HEIGHT * 2.0;

pub struct Character {
    name: &'static str,
    // It is possible to get the body handle from the collider handle following the example below
    //      Assuming `collider_handle` is a valid handle of a collider previously added to the world.
    //      let collider = collider_set.get(collider_handle).expect("Collider not found.");
    //      let body_handle = collider.body();
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
    shape: Rectangle,
}

impl Character {
    pub fn new(
        body_set: &mut DefaultBodySet<f64>,
        collider_set: &mut DefaultColliderSet<f64>,
    ) -> Character {
        let character_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            CHARACTER_BODY_WIDTH,
            CHARACTER_BODY_HEIGHT,
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
            shape: Rectangle::new([0.0, 0.0, 0.0, 1.0]), // Black color
        }
    }
}

impl Renderable for Character {
    fn render<G: Graphics>(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut G,
        world: &DefaultBodySet<f64>,
    ) {
        if let Some(body) = world.rigid_body(self.body_handle) {
            let character_body = body.borrow();
            let position = character_body.position().translation.vector;
            self.shape.draw(
                [
                    position[0] - CHARACTER_BODY_WIDTH,
                    position[1] - CHARACTER_BODY_HEIGHT,
                    CHARACTER_RENDER_WIDTH,
                    CHARACTER_RENDER_HEIGHT,
                ],
                &context.draw_state,
                transform,
                graphics,
            )
        }
    }
}
