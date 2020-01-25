use crate::game::insertable::Insertable;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::object::{ColliderDesc, RigidBodyDesc};
use opengl_graphics::{Texture, TextureSettings};
use std::rc::Rc;
use uuid::Uuid;

const BULLET_BODY_WIDTH: f64 = 5.0;
const BULLET_BODY_HEIGHT: f64 = 5.0;

#[derive(Clone)]
pub struct Bullet {
    damage: u32,
}

impl Bullet {
    pub fn new(initial_position: (f64, f64)) -> Insertable {
        let bullet_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            BULLET_BODY_WIDTH,
            BULLET_BODY_HEIGHT,
        )));

        let bullet_collider = ColliderDesc::new(bullet_shape).density(0.1);

        let bullet_body = RigidBodyDesc::new()
            .position(Isometry2::translation(
                initial_position.0,
                initial_position.1,
            ))
            .velocity(Velocity2::new(Vector2::new(1.0, 1.0), 0.0))
            .user_data(Bullet { damage: 10 })
            .build();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let tex: Rc<Texture> = Rc::new(
            Texture::from_path(assets.join("vaccine.png"), &TextureSettings::new()).unwrap(),
        );

        Insertable::new(tex, bullet_body, Some(bullet_collider))
    }

    pub fn damage(&self) -> u32 {
        self.damage
    }
}
