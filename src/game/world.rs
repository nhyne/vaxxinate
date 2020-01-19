use crate::game::player::character::Character;
use crate::game::renderable::Renderable;
use nalgebra::geometry::Rotation2;
use nalgebra::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use piston_window::math::Matrix2d;
use piston_window::Motion::*;
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

    pub fn handle_mouse(&self, motion: Motion) {
        match motion {
            Motion::MouseCursor(motion) => println!("{:#?}", motion),
            _ => {}
        }
        // Want to change the rotation of the player
    }

    pub fn handle_key_press(&self, key: ButtonArgs) {
        println!("{:#?}", key)
    }

    fn calculate_player_rotation(&self, mouse_position: [f64; 2]) {}
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
