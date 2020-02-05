use crate::game::enemy::Enemy;
use crate::game::insertable::Insertable;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{ColliderDesc, DefaultBodyHandle, DefaultBodySet, RigidBodyDesc};
use opengl_graphics::{Texture, TextureSettings};
use sprite::Scene;
use std::rc::Rc;
use uuid::Uuid;

const BABY_BODY_WIDTH: f64 = 50.0;
const BABY_BODY_HEIGHT: f64 = 25.0;

#[derive(Clone)]
pub struct BabyInt {
    pub uuid: Uuid,
}

pub struct Baby {
    body_handle: DefaultBodyHandle,
    sprite_uuid: Uuid,
    health: u32,
}

impl Baby {
    pub fn generate_insertable(_player_position: Vector2<f64>) -> Insertable {
        // TODO: This should be a multi part rigid body to make the collisions better
        let baby_shape =
            ShapeHandle::new(Cuboid::new(Vector2::new(BABY_BODY_WIDTH, BABY_BODY_HEIGHT)));

        let baby_collider = ColliderDesc::new(baby_shape).density(0.1);

        let baby_body = RigidBodyDesc::new()
            // TODO: Position should be a random position that is at least a
            //  certain distance away from the player.
            .position(Isometry2::translation(250.0, 250.0))
            .user_data(BabyInt {
                uuid: Uuid::new_v4(),
            })
            .build();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let tex: Rc<Texture> =
            Rc::new(Texture::from_path(assets.join("baby.png"), &TextureSettings::new()).unwrap());

        Insertable::new(tex, baby_body, Some(baby_collider))
    }

    pub fn new(sprite_uuid: Uuid, body_handle: DefaultBodyHandle) -> Self {
        Baby {
            sprite_uuid,
            body_handle,
            health: 32,
        }
    }

    pub fn update(&self, world: &DefaultBodySet<f64>, scene: &mut Scene<Texture>) {
        if let Some(bullet_sprite) = scene.child_mut(self.sprite_uuid) {
            if let Some(rigid_body) = world.rigid_body(self.body_handle) {
                let rigid_body_pos = rigid_body.position().translation.vector;
                let (x_pos, y_pos) = (rigid_body_pos[0], rigid_body_pos[1]);
                bullet_sprite.set_position(x_pos, y_pos);

                //                bullet_sprite.set_rotation(rigid_body.position().rotation.angle() * 57.29578);
            }
        }
    }
}

impl Enemy for Baby {
    fn spawn() {}
}
