use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use super::player::player::Player;
use nalgebra::Vector2;

pub struct World {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometric_world: DefaultGeometricalWorld<f64>,
    player: Player,
}

impl World {
    pub fn new() -> World {
        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometric_world: DefaultGeometricalWorld::new(),
            player: Player::new(),
        }
    }
}
