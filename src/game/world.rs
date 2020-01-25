use crate::game::bullet::Bullet;
use crate::game::insertable::Insertable;
use crate::game::player::character::Character;
use nalgebra::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{BodyPartHandle, DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::math::Matrix2d;
use piston_window::{clear, Button, ButtonArgs, ButtonState, Context, Graphics, Key, Motion};
use sprite::{Scene, Sprite};
use std::collections::HashSet;

pub struct World {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometric_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    collider_set: DefaultColliderSet<f64>,
    force_set: DefaultForceGeneratorSet<f64>,
    joint_constraint_set: DefaultJointConstraintSet<f64>,
    scene: Scene<Texture>,
    character: Character,
    keys_pressed: HashSet<Key>,
    mouse_position: [f64; 2],
}

impl World {
    pub fn new() -> World {
        let mut scene: Scene<Texture> = Scene::new();
        let mut body_set: DefaultBodySet<f64> = DefaultBodySet::new();
        let mut collider_set: DefaultColliderSet<f64> = DefaultColliderSet::new();
        let force_set: DefaultForceGeneratorSet<f64> = DefaultForceGeneratorSet::new();
        let joint_constraint_set: DefaultJointConstraintSet<f64> = DefaultJointConstraintSet::new();
        let character =
            Character::new(&mut body_set, &mut collider_set, (100.0, 100.0), &mut scene);
        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometric_world: DefaultGeometricalWorld::new(),
            body_set,
            collider_set,
            force_set,
            joint_constraint_set,
            character,
            keys_pressed: HashSet::new(),
            mouse_position: [0.0, 0.0],
            scene,
        }
    }

    pub fn insert_into_world(&mut self, to_insert: Insertable) {
        // TODO: Drop the Insertable
        // Here I want all of the resources the Insertable owns to be passed to the functions
        // The Insertable should not own anything anymore
        let initial_position = to_insert.rigid_body.position();
        let inserted_handle = self.body_set.insert(to_insert.rigid_body);

        let mut sprite = Sprite::from_texture(to_insert.texture);
        sprite.set_position(100.0, 100.0);
        self.scene.add_child(sprite);

        if let Some(collider_desc) = to_insert.collider_desc {
            let collider = collider_desc.build(BodyPartHandle(inserted_handle, 0));
            let _collider_handle = self.collider_set.insert(collider);
        }
    }

    pub fn update(&mut self) {
        self.character
            .update(&mut self.body_set, &self.keys_pressed, &mut self.scene);
        self.mechanical_world.step(
            &mut self.geometric_world,
            &mut self.body_set,
            &mut self.collider_set,
            &mut self.joint_constraint_set,
            &mut self.force_set,
        );
        self.character
            .update_rotation(self.mouse_position, &mut self.body_set);
    }

    pub fn render(&self, _context: Context, transform: Matrix2d, graphics: &mut GlGraphics) {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.scene.draw(transform, graphics);
    }

    pub fn handle_mouse(&mut self, motion: Motion) {
        // Want to change the rotation of the player
        // Should just set the rotation of the player and the the player render function actually handle rendering
        if let Motion::MouseCursor(motion) = motion {
            self.mouse_position = motion;
            self.character.update_rotation(motion, &mut self.body_set);
        }
    }

    pub fn handle_button_event(&mut self, key: ButtonArgs) {
        match key.state {
            ButtonState::Press => {
                match key.button {
                    Button::Keyboard(key) => {
                        self.keys_pressed.insert(key);
                    }
                    Button::Mouse(mouse_button) => {
                        // here we want to spawn a bullet. Not sure what to do on release? -- figure out automatic weapons later?
                        let bullet = Bullet::new((125.0, 125.0));
                        self.insert_into_world(bullet);
                    }
                    _ => {}
                }
            }
            ButtonState::Release => {
                if let Button::Keyboard(key) = key.button {
                    self.keys_pressed.remove(&key);
                }
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
