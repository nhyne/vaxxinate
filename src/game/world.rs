use crate::game::bullet::{Bullet, InsertedBullet};
use crate::game::enemy::baby::Baby;
use crate::game::insertable::Insertable;
use crate::game::player::character::Character;
use input::MouseButton;
use nalgebra::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{BodyPartHandle, DefaultBodySet, DefaultColliderSet, DefaultBodyHandle};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::math::Matrix2d;
use piston_window::{clear, Button, ButtonArgs, ButtonState, Context, Graphics, Key, Motion};
use sprite::{Scene, Sprite};
use std::collections::HashSet;
use ncollide2d::narrow_phase::ContactEvent;

pub struct World {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometric_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    collider_set: DefaultColliderSet<f64>,
    force_set: DefaultForceGeneratorSet<f64>,
    joint_constraint_set: DefaultJointConstraintSet<f64>,
    scene: Scene<Texture>,
    character: Character,
    babies: Vec<Baby>,
    bullets: Vec<InsertedBullet>,
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

        // Temporary code
        let baby_insertable = Baby::generate_insertable(Vector2::new(10.0, 10.0));
        let baby_handle = body_set.insert(baby_insertable.rigid_body);
        let mut baby_sprite = Sprite::from_texture(baby_insertable.texture);
        baby_sprite.set_position(250.0, 250.0);
        let baby_id = scene.add_child(baby_sprite);

        if let Some(collider_desc) = baby_insertable.collider_desc {
            let collider = collider_desc.build(BodyPartHandle(baby_handle, 0));
            let _collider_handle = collider_set.insert(collider);
        }

        let test_baby = Baby::new(baby_id, baby_handle);

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
            bullets: vec![],
            babies: vec![test_baby],
        }
    }

    pub fn insert_into_world(&mut self, to_insert: Insertable) {
        // TODO: Drop the Insertable
        // Here I want all of the resources the Insertable owns to be passed to the functions
        // The Insertable should not own anything anymore
        let _initial_position = to_insert.rigid_body.position();
        let inserted_handle = self.body_set.insert(to_insert.rigid_body);

        let sprite = Sprite::from_texture(to_insert.texture);
        let sprite_uuid = self.scene.add_child(sprite);

        if let Some(collider_desc) = to_insert.collider_desc {
            let collider = collider_desc.build(BodyPartHandle(inserted_handle, 0));
            let _collider_handle = self.collider_set.insert(collider);
        }

        self.bullets
            .insert(0, InsertedBullet::new(sprite_uuid, inserted_handle));
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
        for bullet in &self.bullets {
            bullet.update(&self.body_set, &mut self.scene);
        }

        for baby in &self.babies {
            baby.update(&self.body_set, &mut self.scene);
        }

        for contact_event in self.geometric_world.contact_events().iter() {
            self.handle_contact_event(contact_event);
        }
    }

    fn handle_contact_event(&self, contact_event: &ContactEvent<DefaultBodyHandle>) {
        match contact_event {
            ContactEvent::Started(first_handle, second_handle) => {
                let body = self.body_set.rigid_body(*first_handle).unwrap();
                println!("FIRST HANDLE: {:#?}, BODY: {:#?}", first_handle, body);
            }
            ContactEvent::Stopped(first_handle, second_handle) => {

            }
        }
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
            ButtonState::Press => match key.button {
                Button::Keyboard(key) => {
                    self.keys_pressed.insert(key);
                }
                Button::Mouse(mouse_button) => {
                    if let MouseButton::Left = mouse_button {
                        let player_position = self.character.get_position(&self.body_set);
                        let player_rotation = self.character.get_rotation(&self.body_set);
                        let bullet = Bullet::generate_insertable(player_position, player_rotation);
                        self.insert_into_world(bullet);
                    }
                }
                _ => {}
            },
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
