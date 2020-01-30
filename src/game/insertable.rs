use nalgebra::Isometry2;
use nphysics2d::object::{ColliderDesc, DefaultBodyHandle, DefaultColliderHandle, RigidBody};
use opengl_graphics::Texture;
use std::rc::Rc;
use uuid::Uuid;

pub struct PhysicsInsertable {
    rigid_body: RigidBody<f64>,
    // This is optional in case something does not want to collide
    collider_desc: Option<ColliderDesc<f64>>,
}

impl PhysicsInsertable {
    fn new(
        rigid_body: RigidBody<f64>,
        collider_desc: Option<ColliderDesc<f64>>,
    ) -> PhysicsInsertable {
        PhysicsInsertable {
            rigid_body,
            collider_desc,
        }
    }

    fn rigid_body_position(&self) -> &Isometry2<f64> {
        self.rigid_body.position()
    }

    fn parts(self) -> (RigidBody<f64>, Option<ColliderDesc<f64>>) {
        (self.rigid_body, self.collider_desc)
    }
}

pub struct Insertable {
    texture: Rc<Texture>,
    physics_insertable: PhysicsInsertable,
}

impl Insertable {
    pub fn new(
        texture: Rc<Texture>,
        rigid_body: RigidBody<f64>,
        collider_desc: Option<ColliderDesc<f64>>,
    ) -> Insertable {
        let physics_insertable = PhysicsInsertable::new(rigid_body, collider_desc);
        Insertable {
            texture,
            physics_insertable,
        }
    }

    fn rigid_body_position(&self) -> &Isometry2<f64> {
        self.physics_insertable.rigid_body_position()
    }

    pub fn get_parts(self) -> (Rc<Texture>, RigidBody<f64>, Option<ColliderDesc<f64>>) {
        let (rigid_body, collider_desc) = self.physics_insertable.parts();
        (self.texture, rigid_body, collider_desc)
    }
}

pub struct Inserted {
    sprite_uuid: Uuid,
    body_handle: DefaultBodyHandle,
    collider_handle: Option<DefaultColliderHandle>,
}

impl Inserted {
    pub fn new(
        sprite_uuid: Uuid,
        body_handle: DefaultBodyHandle,
        collider_handle: Option<DefaultColliderHandle>,
    ) -> Inserted {
        Inserted {
            sprite_uuid,
            body_handle,
            collider_handle,
        }
    }
}
