use crate::game::insertable::{Insertable, Inserted, InsertedType};
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::object::{ColliderDesc, DefaultBodySet, RigidBody, RigidBodyDesc};
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
    inserted: Inserted,
}

impl Bullet {
    // takes rotation in RADIANS
    pub fn generate_insertable(
        initial_position: Vector2<f64>,
        rotation_rad: f64,
    ) -> (Insertable, Uuid) {
        let bullet_uuid = Uuid::new_v4();
        let bullet_collider = Bullet::generate_bullet_collider_desc();
        let bullet_body = Bullet::generate_bullet_body(initial_position, rotation_rad, bullet_uuid);
        let tex = Bullet::generate_bullet_texture();

        (
            Insertable::new(tex, bullet_body, Some(bullet_collider)),
            bullet_uuid,
        )
    }

    fn generate_bullet_collider_desc() -> ColliderDesc<f64> {
        let bullet_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            BULLET_BODY_WIDTH,
            BULLET_BODY_HEIGHT,
        )));

        ColliderDesc::new(bullet_shape).density(0.1)
    }

    fn generate_bullet_texture() -> Rc<Texture> {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        Rc::new(Texture::from_path(assets.join("vaccine.png"), &TextureSettings::new()).unwrap())
    }

    fn generate_bullet_body(
        initial_position: Vector2<f64>,
        rotation_rad: f64,
        bullet_uuid: Uuid,
    ) -> RigidBody<f64> {
        let directional_unit_vector = Bullet::bullet_directional_unit_vector(rotation_rad);
        let velocity_vector: Vector2<f64> = Vector2::new(
            directional_unit_vector[0] * BULLET_SPEED,
            directional_unit_vector[1] * BULLET_SPEED,
        );

        RigidBodyDesc::new()
            .position(Isometry2::translation(
                initial_position[0] + BULLET_SPAWN_OFFSET * directional_unit_vector[0],
                initial_position[1] + BULLET_SPAWN_OFFSET * directional_unit_vector[1],
            ))
            .velocity(Velocity2::new(velocity_vector, 0.0))
            .user_data(InsertedType::Bullet(bullet_uuid))
            .max_angular_velocity(0.0)
            .rotation(rotation_rad)
            .build()
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
    pub fn new(inserted: Inserted) -> Self {
        InsertedBullet { inserted }
    }

    pub fn update(&self, world: &DefaultBodySet<f64>, scene: &mut Scene<Texture>) {
        if let Some(bullet_sprite) = scene.child_mut(self.inserted.get_sprite_uuid()) {
            if let Some(rigid_body) = world.rigid_body(self.inserted.get_body_handle()) {
                let rigid_body_pos = rigid_body.position().translation.vector;
                let (x_pos, y_pos) = (rigid_body_pos[0], rigid_body_pos[1]);
                bullet_sprite.set_position(x_pos, y_pos);

                bullet_sprite.set_rotation(rigid_body.position().rotation.angle() * 57.29578);
            }
        }
    }
}
