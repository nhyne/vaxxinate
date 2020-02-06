use nalgebra::Isometry2;
use nphysics2d::object::{
    ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle, RigidBody,
};
use opengl_graphics::Texture;
use sprite::Scene;
use std::rc::Rc;
use uuid::Uuid;

pub struct PhysicsInsertable {
    rigid_body: RigidBody<f64>,
    // This is optional in case something does not want to collide
    collider_desc: Option<ColliderDesc<f64>>,
}

/// Represents and object that can be inserted into the physics world.
/// When inserted into the world a PhysicsInsertable should be returned to keep track of the objects in the physics world.
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

    pub fn parts(self) -> (RigidBody<f64>, Option<ColliderDesc<f64>>) {
        (self.rigid_body, self.collider_desc)
    }
}

pub struct PhysicsInserted {
    body_handle: DefaultBodyHandle,
    collider_handle: Option<DefaultColliderHandle>,
}

/// Represents an object that has been inserted into the physics world.
impl PhysicsInserted {
    pub fn new(
        body_handle: DefaultBodyHandle,
        collider_handle: Option<DefaultColliderHandle>,
    ) -> PhysicsInserted {
        PhysicsInserted {
            body_handle,
            collider_handle,
        }
    }

    fn get_body_handle(&self) -> DefaultBodyHandle {
        self.body_handle
    }
}

pub struct Insertable {
    texture: Rc<Texture>,
    physics_insertable: PhysicsInsertable,
}

/// Represents and object that can be inserted into the physics and sprite worlds.
/// When inserted into the world an Inserted should be returned to track the sprite and physics objects.
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

    pub fn get_parts_insertable(self) -> (Rc<Texture>, PhysicsInsertable) {
        (self.texture, self.physics_insertable)
    }
}

/// Represents an item that has been inserted into the physics and sprite worlds.
/// When two colliders make contact, there should be a Uuid in the user_data of that collider which will point to an Inserted.
/// Based on actions we need to take, we can apply affects to the Inserted by referencing its handles and sprite Uuid.
pub struct Inserted {
    sprite_uuid: Uuid,
    physics_inserted: PhysicsInserted,
}

impl Inserted {
    pub fn new(
        sprite_uuid: Uuid,
        body_handle: DefaultBodyHandle,
        collider_handle: Option<DefaultColliderHandle>,
    ) -> Inserted {
        let physics_inserted = PhysicsInserted::new(body_handle, collider_handle);
        Inserted {
            sprite_uuid,
            physics_inserted,
        }
    }

    pub fn new_from_physics(sprite_uuid: Uuid, physics_inserted: PhysicsInserted) -> Inserted {
        Inserted {
            sprite_uuid,
            physics_inserted,
        }
    }

    pub fn get_sprite_uuid(&self) -> Uuid {
        self.sprite_uuid
    }

    pub fn get_body_handle(&self) -> DefaultBodyHandle {
        self.physics_inserted.body_handle
    }
}

pub trait InsertedBody {
    fn update(&self, world: &DefaultBodySet<f64>, scene: &mut Scene<Texture>);
    fn get_body_handle(&self) -> DefaultBodyHandle;
    fn get_sprite_uuid(&self) -> Uuid;
}
