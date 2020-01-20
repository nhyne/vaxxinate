use crate::game::renderable::Renderable;

use graphics::{image, Context};
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::{Force2, ForceType};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{
    Body, BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet,
    RigidBodyDesc,
};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston_window::math::Matrix2d;
use piston_window::{Key, Rectangle, Transformed};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::path::Path;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CHARACTER_BODY_WIDTH: f64 = 20.0;
const CHARACTER_BODY_HEIGHT: f64 = 20.0;
const CHARACTER_RENDER_WIDTH: f64 = CHARACTER_BODY_WIDTH * 2.0;
const CHARACTER_RENDER_HEIGHT: f64 = CHARACTER_BODY_HEIGHT * 2.0;

pub struct Character {
    // It is possible to get the body handle from the collider handle following the example below
    //      Assuming `collider_handle` is a valid handle of a collider previously added to the world.
    //      let collider = collider_set.get(collider_handle).expect("Collider not found.");
    //      let body_handle = collider.body();
    body_handle: DefaultBodyHandle,
    //    collider_handle: DefaultColliderHandle,
    shape: Rectangle,
    player_image: Texture,
}

impl Character {
    pub fn new(
        body_set: &mut DefaultBodySet<f64>,
        collider_set: &mut DefaultColliderSet<f64>,
        position: (f64, f64),
    ) -> Character {
        let character_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            CHARACTER_BODY_WIDTH,
            CHARACTER_BODY_HEIGHT,
        )));

        let character_collider = ColliderDesc::new(character_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));

        let character_body = RigidBodyDesc::new()
            .position(Isometry2::translation(position.0, position.1))
            .linear_damping(1.0)
            .build();

        let body_handle = body_set.insert(character_body);

        let character_collider = character_collider.build(BodyPartHandle(body_handle, 0));
        let _collider_handle = collider_set.insert(character_collider);

        let player_image =
            Texture::from_path(&Path::new("./assets/player.png"), &TextureSettings::new()).unwrap();

        Character {
            body_handle,
            //            collider_handle,
            shape: Rectangle::new(BLACK),
            player_image,
        }
    }

    pub fn update(&mut self, world: &mut DefaultBodySet<f64>, keys_pressed: &HashSet<Key>) {
        if keys_pressed.contains(&Key::W) {
            self.move_up(world);
        }
        if keys_pressed.contains(&Key::A) {
            self.move_left(world);
        }
        if keys_pressed.contains(&Key::D) {
            self.move_right(world);
        }
        if keys_pressed.contains(&Key::S) {
            self.move_down(world);
        }
    }

    pub fn update_rotation(&mut self, mouse_position: [f64; 2], world: &mut DefaultBodySet<f64>) {
        //        use std::f64;
        if let Some(character_body) = world.rigid_body_mut(self.body_handle) {
            let position = character_body.position().translation.vector;
            let (char_x_pos, char_y_pos) = (position[0], position[1]);
            let [mouse_x_pos, mouse_y_pos] = mouse_position;
            let x_diff = mouse_x_pos - char_x_pos;
            let y_diff = mouse_y_pos - char_y_pos;

            // Not sure why this is negative. The cube rotated the opposite direction without it
            let tangent = -(x_diff / y_diff).atan();
            character_body.set_position(Isometry2::new(position, tangent));
        }
    }

    fn move_left(&self, world: &mut DefaultBodySet<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body_handle) {
            let force = Force2::linear(Vector2::new(-5.0, 0.0));
            body.apply_force(0, &force, ForceType::VelocityChange, false);
        }
    }

    fn move_right(&self, world: &mut DefaultBodySet<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body_handle) {
            let force = Force2::linear(Vector2::new(5.0, 0.0));
            body.apply_force(0, &force, ForceType::VelocityChange, false);
        }
    }

    fn move_down(&self, world: &mut DefaultBodySet<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body_handle) {
            let force: Force2<f64> = Force2::linear(Vector2::new(0.0, 5.0));
            body.apply_force(0, &force, ForceType::VelocityChange, false);
        }
    }

    fn move_up(&self, world: &mut DefaultBodySet<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body_handle) {
            let jump_force = Force2::linear(Vector2::new(0.0, -5.0));
            body.apply_force(0, &jump_force, ForceType::VelocityChange, false);
        }
    }
}

impl Renderable for Character {
    fn render(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut GlGraphics,
        world: &DefaultBodySet<f64>,
    ) {
        if let Some(body) = world.rigid_body(self.body_handle) {
            //TODO Cleanup this function
            let character_body = body.borrow();
            let position = character_body.position().translation.vector;
            let (x_pos, y_pos) = (position[0], position[1]);
            let rotation = character_body.position().rotation.angle();
            let rotation_transform = transform
                .trans(x_pos, y_pos)
                .rot_rad(rotation)
                .trans(-x_pos, -y_pos);
            self.shape.draw(
                [
                    x_pos - CHARACTER_BODY_WIDTH,
                    y_pos - CHARACTER_BODY_HEIGHT,
                    CHARACTER_RENDER_WIDTH,
                    CHARACTER_RENDER_HEIGHT,
                ],
                &context.draw_state,
                rotation_transform,
                graphics,
            );
            image(&self.player_image, rotation_transform, graphics);
        }
    }
}
