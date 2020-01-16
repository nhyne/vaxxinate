use crate::game::player::character::Character;
use crate::game::view::View;
use nalgebra::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use piston_window::PistonWindow;

pub struct World {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometric_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    collider_set: DefaultColliderSet<f64>,
    character: Character,
    view: View,
}

impl World {
    pub fn new() -> World {
        let mut body_set: DefaultBodySet<f64> = DefaultBodySet::new();
        let mut collider_set: DefaultColliderSet<f64> = DefaultColliderSet::new();
        let character = Character::new(&mut body_set, &mut collider_set);
        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometric_world: DefaultGeometricalWorld::new(),
            body_set,
            collider_set,
            character,
            view: View::default(),
        }
    }

    pub fn window(&mut self) -> &mut PistonWindow {
        &mut self.view.window
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
