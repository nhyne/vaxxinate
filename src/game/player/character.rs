use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::{Force2, ForceType};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{
    Body, BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet,
    RigidBodyDesc,
};
use opengl_graphics::{Texture, TextureSettings};
use piston_window::Key;
use sprite::{Scene, Sprite};
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

const CHARACTER_BODY_WIDTH: f64 = 20.0;
const CHARACTER_BODY_HEIGHT: f64 = 20.0;

pub struct Character {
    // It is possible to get the body handle from the collider handle following the example below
    //      Assuming `collider_handle` is a valid handle of a collider previously added to the world.
    //      let collider = collider_set.get(collider_handle).expect("Collider not found.");
    //      let body_handle = collider.body();
    body_handle: DefaultBodyHandle,
    //    collider_handle: DefaultColliderHandle,
    //    player_image: Texture,
    sprite_uuid: Uuid,
}

impl Character {
    pub fn new(
        body_set: &mut DefaultBodySet<f64>,
        collider_set: &mut DefaultColliderSet<f64>,
        initial_position: (f64, f64),
        scene: &mut Scene<Texture>,
    ) -> Character {
        let character_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            CHARACTER_BODY_WIDTH,
            CHARACTER_BODY_HEIGHT,
        )));

        let character_collider = ColliderDesc::new(character_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));

        let character_body = RigidBodyDesc::new()
            .position(Isometry2::translation(
                initial_position.0,
                initial_position.1,
            ))
            .linear_damping(1.0)
            .build();

        let body_handle = body_set.insert(character_body);

        let character_collider = character_collider.build(BodyPartHandle(body_handle, 0));
        let _collider_handle = collider_set.insert(character_collider);

        let sprite_uuid = Character::generate_sprite(scene, initial_position);

        Character {
            body_handle,
            //            collider_handle,
            sprite_uuid,
        }
    }

    fn generate_sprite(scene: &mut Scene<Texture>, position: (f64, f64)) -> Uuid {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let tex: Rc<Texture> = Rc::new(
            Texture::from_path(assets.join("player.png"), &TextureSettings::new()).unwrap(),
        );
        let mut sprite = Sprite::from_texture(tex);
        sprite.set_position(position.0, position.1);
        scene.add_child(sprite)
    }

    pub fn update(
        &mut self,
        world: &mut DefaultBodySet<f64>,
        keys_pressed: &HashSet<Key>,
        scene: &mut Scene<Texture>,
    ) {
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

        if let Some(char_sprite) = scene.child_mut(self.sprite_uuid) {
            if let Some(rigid_body) = world.rigid_body(self.body_handle) {
                let rigid_body_pos = rigid_body.position().translation.vector;
                let (x_pos, y_pos) = (rigid_body_pos[0], rigid_body_pos[1]);
                char_sprite.set_position(x_pos, y_pos);

                let char_rotation = rigid_body.position().rotation.angle();
                // char_rotation comes back in radians, converting to degrees (what set_rotation expects)
                let rotation = char_rotation * 57.29578;
                char_sprite.set_rotation(rotation);
            }
        }
    }

    pub fn update_rotation(&mut self, mouse_position: [f64; 2], world: &mut DefaultBodySet<f64>) {
        use std::f64;
        if let Some(character_body) = world.rigid_body_mut(self.body_handle) {
            let position = character_body.position().translation.vector;
            let (char_x_pos, char_y_pos) = (position[0], position[1]);
            let [mouse_x_pos, mouse_y_pos] = mouse_position;
            let x_diff = mouse_x_pos - char_x_pos;
            let y_diff = mouse_y_pos - char_y_pos;

            let mut tangent = (y_diff / x_diff).atan();
            // All the tangent addition/subtraction was trial and error
            if x_diff >= 0.0 {
                // We're in the first quadrant (bottom right of player) or the fourth quadrant (top right of player)
                tangent += f64::consts::FRAC_PI_2;
            } else {
                // We're in the second quadrant (bottom left of player) or the third quadrant (top left of player)
                tangent -= f64::consts::FRAC_PI_2;
            }
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
