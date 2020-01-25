use nphysics2d::object::{ColliderDesc, RigidBody};
use opengl_graphics::Texture;
use std::rc::Rc;

pub struct Insertable {
    pub texture: Rc<Texture>,
    pub rigid_body: RigidBody<f64>,
    // This is optional in case something does not want to collide
    pub collider_desc: Option<ColliderDesc<f64>>,
}

impl Insertable {
    pub fn new(
        texture: Rc<Texture>,
        rigid_body: RigidBody<f64>,
        collider_desc: Option<ColliderDesc<f64>>,
    ) -> Insertable {
        Insertable {
            texture,
            rigid_body,
            collider_desc,
        }
    }
}
