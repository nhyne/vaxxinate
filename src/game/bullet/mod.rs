use crate::game::insertable::Insertable;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::object::{ColliderDesc, DefaultBodyHandle, DefaultBodySet, RigidBodyDesc};
use opengl_graphics::{Texture, TextureSettings};
use sprite::Scene;
use std::rc::Rc;
use uuid::Uuid;

const BULLET_BODY_WIDTH: f64 = 5.0;
const BULLET_BODY_HEIGHT: f64 = 5.0;
const BULLET_SPAWN_OFFSET: f64 = 35.0;
const BULLET_SPEED: f64 = 250.0;

#[derive(Clone)]
pub struct Bullet {
    damage: u32,
}

pub struct InsertedBullet {
    sprite_uuid: Uuid,
    body_handle: DefaultBodyHandle,
}

impl Bullet {
    // takes rotation in RADIANS
    pub fn generate_insertable(initial_position: Vector2<f64>, rotation_rad: f64) -> Insertable {
        let bullet_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            BULLET_BODY_WIDTH,
            BULLET_BODY_HEIGHT,
        )));

        let bullet_collider = ColliderDesc::new(bullet_shape).density(0.1);

        let directional_unit_vector = Bullet::bullet_directional_unit_vector(rotation_rad);
        let velocity_vector: Vector2<f64> = Vector2::new(
            directional_unit_vector[0] * BULLET_SPEED,
            directional_unit_vector[1] * BULLET_SPEED,
        );

        let bullet_body = RigidBodyDesc::new()
            .position(Isometry2::translation(
                initial_position[0] + BULLET_SPAWN_OFFSET * directional_unit_vector[0],
                initial_position[1] + BULLET_SPAWN_OFFSET * directional_unit_vector[1],
            ))
            .velocity(Velocity2::new(velocity_vector, 0.0))
            .user_data(Bullet { damage: 10 })
            .max_angular_velocity(0.0)
            .rotation(rotation_rad)
            .build();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let tex: Rc<Texture> = Rc::new(
            Texture::from_path(assets.join("vaccine.png"), &TextureSettings::new()).unwrap(),
        );

        Insertable::new(tex, bullet_body, Some(bullet_collider))
    }

    fn bullet_directional_unit_vector(rotation_rad: f64) -> Vector2<f64> {
        use std::f64;

        let corrected_rotation = rotation_rad - f64::consts::FRAC_PI_2;
        let x_addition = corrected_rotation.cos();
        let y_addition = corrected_rotation.sin();

        Vector2::new(x_addition, y_addition)
    }
}

impl InsertedBullet {
    pub fn new(sprite_uuid: Uuid, body_handle: DefaultBodyHandle) -> Self {
        InsertedBullet {
            sprite_uuid,
            body_handle,
        }
    }

    pub fn update(&self, world: &DefaultBodySet<f64>, scene: &mut Scene<Texture>) {
        if let Some(bullet_sprite) = scene.child_mut(self.sprite_uuid) {
            if let Some(rigid_body) = world.rigid_body(self.body_handle) {
                let rigid_body_pos = rigid_body.position().translation.vector;
                let (x_pos, y_pos) = (rigid_body_pos[0], rigid_body_pos[1]);
                bullet_sprite.set_position(x_pos, y_pos);

                bullet_sprite.set_rotation(rigid_body.position().rotation.angle() * 57.29578);
            }
        }
    }
}
