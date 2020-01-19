use crate::game::player::character::Character;
use crate::game::renderable::Renderable;
use nalgebra::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use piston_window::math::Matrix2d;
use piston_window::{clear, ButtonArgs, Context, Graphics, Motion};

pub struct World {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometric_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    collider_set: DefaultColliderSet<f64>,
    character: Character,
}

impl World {
    pub fn new() -> World {
        let mut body_set: DefaultBodySet<f64> = DefaultBodySet::new();
        let mut collider_set: DefaultColliderSet<f64> = DefaultColliderSet::new();
        let character = Character::new(&mut body_set, &mut collider_set, (50.0, 50.0));
        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometric_world: DefaultGeometricalWorld::new(),
            body_set,
            collider_set,
            character,
        }
    }

    pub fn render<G: Graphics>(&self, context: Context, transform: Matrix2d, graphics: &mut G) {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.character
            .render(context, transform, graphics, &self.body_set)
    }

    pub fn handle_mouse(&mut self, motion: Motion) {
        match motion {
            Motion::MouseCursor(motion) => {
                println!("Mouse is in motion!: {:#?}", motion);
                self.character.update_rotation(motion, &mut self.body_set);
            }
            _ => {}
        }
        // Want to change the rotation of the player
        // Should just set the rotation of the player and the the player render function actually handle rendering
    }

    pub fn handle_key_press(&self, key: ButtonArgs) {
        println!("{:#?}", key)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
