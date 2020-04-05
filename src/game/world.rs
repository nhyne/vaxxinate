use crate::config::settings::Settings;
use crate::game::bullet::{BulletUserData, InsertedBullet};
use crate::game::enemy::baby::{BabyUserData, InsertedBaby};
use crate::game::insertable::{Insertable, Inserted, InsertedBody};
use crate::game::physics_world::PhysicsWorld;
use crate::game::player::character::Character;
use input::MouseButton;
use nalgebra::Vector2;
use ncollide2d::narrow_phase::ContactEvent;
use nphysics2d::object::{BodyPartHandle, DefaultBodySet, DefaultColliderSet};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::math::Matrix2d;
use piston_window::{clear, Button, ButtonArgs, ButtonState, Context, Graphics, Key, Motion};
use sprite::{Scene, Sprite};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use uuid::Uuid;

/// World struct. Contains the physics world, sprite scene, and other things that we need to keep track of and react to during the game loop.
pub struct World {
    physics_world: PhysicsWorld,
    scene: Scene<Texture>,
    character: Character,
    babies: HashMap<Uuid, InsertedBaby>,
    bullets: HashMap<Uuid, InsertedBullet>,
    keys_pressed: HashSet<Key>,
    mouse_position: [f64; 2],
}

impl World {
    pub fn new(config: &Settings) -> World {
        let mut scene: Scene<Texture> = Scene::new();
        let mut physics_world = PhysicsWorld::new();
        let (body_set, collider_set) = physics_world.body_collider_sets_mut();
        let character = Character::new(
            body_set,
            collider_set,
            (config.player.spawn_point.x, config.player.spawn_point.y),
            &mut scene,
        );

        let (test_baby, test_baby_uuid) = World::insert_baby(&mut scene, body_set, collider_set);
        let mut babies = HashMap::new();
        babies.insert(test_baby_uuid, test_baby);

        World {
            physics_world,
            character,
            keys_pressed: HashSet::new(),
            mouse_position: [0.0, 0.0],
            scene,
            bullets: HashMap::new(),
            babies,
        }
    }

    // TODO: This should probably just insert the baby? Or something else should determine when to spawn.
    fn insert_baby(
        scene: &mut Scene<Texture>,
        body_set: &mut DefaultBodySet<f64>,
        collider_set: &mut DefaultColliderSet<f64>,
    ) -> (InsertedBaby, Uuid) {
        // Temporary code
        let (baby_insertable, uuid) = BabyUserData::generate_insertable(Vector2::new(10.0, 10.0));

        let (sprite_tex, rigid_body, collider_desc_option) = baby_insertable.get_parts();
        let mut baby_sprite = Sprite::from_texture(sprite_tex);
        baby_sprite.set_position(249.0, 250.0);
        let baby_id = scene.add_child(baby_sprite);

        let baby_handle = body_set.insert(rigid_body);
        match collider_desc_option {
            Some(collider_desc) => {
                let collider = collider_desc.build(BodyPartHandle(baby_handle, 0));
                let collider_handle = collider_set.insert(collider);
                (
                    InsertedBaby::new(baby_id, baby_handle, Some(collider_handle)),
                    uuid,
                )
            }
            None => (InsertedBaby::new(baby_id, baby_handle, None), uuid),
        }
    }

    /// Steps the physics world forward.
    pub fn step(&mut self) {
        self.physics_world.step();
    }

    /// Updates all of the parts of the world that change during steps.
    pub fn update(&mut self) {
        let (body_set, _) = self.physics_world.body_collider_sets_mut();
        let scene = &mut self.scene;
        self.character.update(body_set, &self.keys_pressed, scene);

        self.character
            .update_rotation(self.mouse_position, body_set);

        let _: Vec<_> = self
            .bullets
            .values()
            .map(|b| {
                b.update(body_set, scene);
            })
            .collect();

        let _: Vec<_> = self
            .babies
            .values()
            .map(|b| {
                b.update(body_set, scene);
            })
            .collect();

        self.handle_contact_events();
    }

    // TODO: Too long to calculate these events in every loop
    // Actually dealing with these contact events takes a long time.
    //  Long enough that the game is taking too long to spawn a bullet on click and delays it to the next click.
    //  May have to do this less frequently?
    //  During a different game event?
    /// Handles effects that contact events have on the world.
    /// Used to de-spawn bullets when they collide and other events that occur when two things collide.
    fn handle_contact_events(&mut self) {
        let mut bullets_to_remove: Vec<Uuid> = vec![];
        let mut babies_to_remove: Vec<Uuid> = vec![];
        for contact_event in self.physics_world.geometric_world().contact_events() {
            match contact_event {
                ContactEvent::Started(first_handle, second_handle) => {
                    let first_body_option = self.physics_world.body_set().rigid_body(*first_handle);
                    let second_body_option =
                        self.physics_world.body_set().rigid_body(*second_handle);

                    if let (Some(first_body), Some(second_body)) =
                        (first_body_option, second_body_option)
                    {
                        if let (Some(first_data), Some(second_data)) =
                            (first_body.user_data(), second_body.user_data())
                        {
                            let (first_bullet, first_baby) = (
                                first_data.downcast_ref::<BulletUserData>(),
                                first_data.downcast_ref::<BabyUserData>(),
                            );
                            let (second_bullet, second_baby) = (
                                second_data.downcast_ref::<BulletUserData>(),
                                second_data.downcast_ref::<BabyUserData>(),
                            );
                            match (first_bullet, first_baby, second_bullet, second_baby) {
                                (Some(bullet), None, None, Some(baby))
                                | (None, Some(baby), Some(bullet), None) => {
                                    bullets_to_remove.insert(0, bullet.uuid);
                                    babies_to_remove.insert(0, baby.uuid);
                                }
                                (_, _, _, _) => {}
                            }
                        }
                    }
                }
                ContactEvent::Stopped(_first_handle, _second_handle) => {}
            }
        }
        for bullet_to_remove in bullets_to_remove {
            if let Some(bullet_removed) = self.bullets.remove(&bullet_to_remove) {
                self.physics_world
                    .body_set_mut()
                    .remove(bullet_removed.get_body_handle());
                self.scene.remove_child(bullet_removed.get_sprite_uuid());
            }
        }
        for baby_to_remove in babies_to_remove {
            if let Some(baby_removed) = self.babies.remove(&baby_to_remove) {
                self.physics_world
                    .body_set_mut()
                    .remove(baby_removed.get_body_handle());
                self.scene.remove_child(baby_removed.get_sprite_uuid());
            }
        }
    }

    /// Render the game. Currently we just need to clear the stencil and let the sprite world draw.
    pub fn render(&self, _context: Context, transform: Matrix2d, graphics: &mut GlGraphics) {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);
        self.scene.draw(transform, graphics);
    }

    pub fn handle_mouse(&mut self, motion: Motion) {
        if let Motion::MouseCursor(motion) = motion {
            let body_set = self.physics_world.body_set_mut();
            self.mouse_position = motion;
            self.character.update_rotation(motion, body_set);
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
                        self.left_mouse_click();
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
        let id = self.insert_sprite(sprite_tex);

        let physics_inserted = self.physics_world.insert(physics_insertable);
        Inserted::new_from_physics(id, physics_inserted)
    }

    fn left_mouse_click(&mut self) {
        let body_set = self.physics_world.body_set();
        let player_position = self.character.get_position(body_set);
        let player_rotation = self.character.get_rotation(body_set);
        let (bullet, bullet_uuid) =
            BulletUserData::generate_insertable(player_position, player_rotation);
        let inserted_bullet = self.insert_insertable(bullet);
        self.bullets
            .insert(bullet_uuid, InsertedBullet::new(inserted_bullet));
    }

    fn insert_sprite(&mut self, sprite_tex: Rc<Texture>) -> Uuid {
        self.scene.add_child(Sprite::from_texture(sprite_tex))
    }
}
