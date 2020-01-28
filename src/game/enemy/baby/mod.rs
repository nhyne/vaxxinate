use crate::game::enemy::Enemy;
use crate::game::insertable::Insertable;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{ColliderDesc, DefaultBodyHandle, RigidBodyDesc};
use opengl_graphics::{Texture, TextureSettings};
use std::rc::Rc;
use uuid::Uuid;

const BABY_BODY_WIDTH: f64 = 150.0;
const BABY_BODY_HEIGHT: f64 = 75.0;

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
            .build();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let tex: Rc<Texture> =
            Rc::new(Texture::from_path(assets.join("baby.png"), &TextureSettings::new()).unwrap());

        Insertable::new(tex, baby_body, Some(baby_collider))
    }

    pub fn update() {}
}

impl Enemy for Baby {
    fn spawn() {}
}
