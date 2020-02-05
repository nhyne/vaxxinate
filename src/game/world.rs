use crate::game::bullet::{Bullet, InsertedBullet};
use crate::game::enemy::baby::{Baby, BabyInt};
use crate::game::insertable::{Insertable, Inserted};
use crate::game::physics_world::PhysicsWorld;
use crate::game::player::character::Character;
use input::MouseButton;
use nalgebra::Vector2;
use ncollide2d::narrow_phase::ContactEvent;
use nphysics2d::object::{BodyPartHandle, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::math::Matrix2d;
use piston_window::{clear, Button, ButtonArgs, ButtonState, Context, Graphics, Key, Motion};
use sprite::{Scene, Sprite};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use uuid::Uuid;

pub struct World {
    physics_world: PhysicsWorld,
    scene: Scene<Texture>,
    character: Character,
    babies: Vec<Baby>,
    bullets: HashMap<Uuid, InsertedBullet>,
    keys_pressed: HashSet<Key>,
    mouse_position: [f64; 2],
}

impl World {
    pub fn new() -> World {
        let mut scene: Scene<Texture> = Scene::new();
        let mut physics_world = PhysicsWorld::new();
        let (body_set, collider_set) = physics_world.body_collider_sets_mut();
        let character = Character::new(body_set, collider_set, (100.0, 100.0), &mut scene);

        let test_baby = World::insert_baby(&mut scene, body_set, collider_set);

        World {
            physics_world,
            character,
            keys_pressed: HashSet::new(),
            mouse_position: [0.0, 0.0],
            scene,
            bullets: HashMap::new(),
            babies: vec![test_baby],
        }
    }

    fn insert_baby(
        scene: &mut Scene<Texture>,
        body_set: &mut DefaultBodySet<f64>,
        collider_set: &mut DefaultColliderSet<f64>,
    ) -> Baby {
        // Temporary code
        let baby_insertable = Baby::generate_insertable(Vector2::new(10.0, 10.0));
        let (sprite_tex, rigid_body, collider_desc_option) = baby_insertable.get_parts();
        let mut baby_sprite = Sprite::from_texture(sprite_tex);
        baby_sprite.set_position(249.0, 250.0);
        let baby_id = scene.add_child(baby_sprite);

        let baby_handle = body_set.insert(rigid_body);
        if let Some(collider_desc) = collider_desc_option {
            let collider = collider_desc.build(BodyPartHandle(baby_handle, 0));
            let _collider_handle = collider_set.insert(collider);
        }

        Baby::new(baby_id, baby_handle)
    }

    pub fn step(&mut self) {
        self.physics_world.step();
    }

    pub fn update(&mut self) {
        let (body_set, _) = self.physics_world.body_collider_sets_mut();
        self.character
            .update(body_set, &self.keys_pressed, &mut self.scene);

        self.character
            .update_rotation(self.mouse_position, body_set);

        // TODO: These should be turned into iterators
        for bullet in self.bullets.values() {
            bullet.update(body_set, &mut self.scene);
        }

        for baby in &self.babies {
            baby.update(body_set, &mut self.scene);
        }

        for contact_event in self.physics_world.geometric_world().contact_events() {
            self.handle_contact_event(contact_event);
        }
    }

    // TODO: Too long to calculate these events in every loop
    // Actually dealing with these contact events takes a long time.
    //  Long enough that the game is taking too long to spawn a bullet on click and delays it to the next click.
    //  May have to do this less frequently?
    //  During a different game event?
    fn handle_contact_event(&self, contact_event: &ContactEvent<DefaultBodyHandle>) {
        match contact_event {
            ContactEvent::Started(first_handle, second_handle) => {
                let first_body_option = self.physics_world.body_set().rigid_body(*first_handle);
                let second_body_option = self.physics_world.body_set().rigid_body(*second_handle);

                if let (Some(first_body), Some(second_body)) =
                    (first_body_option, second_body_option)
                {
                    if let (Some(first_data), Some(second_data)) =
                        (first_body.user_data(), second_body.user_data())
                    {
                        if first_data.type_id() == TypeId::of::<Bullet>()
                            && second_data.type_id() == TypeId::of::<BabyInt>()
                            || first_data.type_id() == TypeId::of::<BabyInt>()
                                && second_data.type_id() == TypeId::of::<Bullet>()
                        {
                            println!("We have a collision that makes us want to do something!!!!!!!!!!!!!!!!!")
                        }
                    }
                }
            }
            ContactEvent::Stopped(_first_handle, _second_handle) => {}
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
            let body_set = self.physics_world.body_set_mut();
            self.mouse_position = motion;
            self.character.update_rotation(motion, body_set);
        }
    }

    pub fn handle_button_event(&mut self, key: ButtonArgs) {
        let body_set = self.physics_world.body_set();
        match key.state {
            ButtonState::Press => match key.button {
                Button::Keyboard(key) => {
                    self.keys_pressed.insert(key);
                }
                Button::Mouse(mouse_button) => {
                    if let MouseButton::Left = mouse_button {
                        let player_position = self.character.get_position(body_set);
                        let player_rotation = self.character.get_rotation(body_set);
                        let (bullet, bullet_uuid) =
                            Bullet::generate_insertable(player_position, player_rotation);
                        let inserted_bullet = self.insert_insertable(bullet);
                        self.bullets
                            .insert(bullet_uuid, InsertedBullet::new(inserted_bullet));
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

    pub fn insert_insertable(&mut self, to_insert: Insertable) -> Inserted {
        let (sprite_tex, physics_insertable) = to_insert.get_parts_insertable();
        // TODO: Does this belong in the Insertable struct or as a param here?
        //       How would a "spawn" call on an enemy type work with this?
        let id = self.insert_sprite(sprite_tex, Vector2::new(250.0, 250.0));

        let physics_inserted = self.physics_world.insert(physics_insertable);
        Inserted::new_from_physics(id, physics_inserted)
    }

    fn insert_sprite(&mut self, sprite_tex: Rc<Texture>, initial_position: Vector2<f64>) -> Uuid {
        let mut sprite = Sprite::from_texture(sprite_tex);
        sprite.set_position(initial_position[0], initial_position[1]);
        self.scene.add_child(sprite)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
