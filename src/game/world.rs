use super::player::character::Character;
use nalgebra::Vector2;
use nphysics2d::object::DefaultBodySet;
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub struct World {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometric_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    character: Character,
}

impl World {
    pub fn new() -> World {
        let mut body_set: DefaultBodySet<f64> = DefaultBodySet::new();
        let character = Character::new(&mut body_set);
        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometric_world: DefaultGeometricalWorld::new(),
            body_set: DefaultBodySet::new(),
            character,
        }
    }
}
