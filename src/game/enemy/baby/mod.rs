use nphysics2d::object::DefaultBodyHandle;
use uuid::Uuid;
use crate::game::enemy::Enemy;

pub struct Baby {
    body_handle: DefaultBodyHandle,
    sprite_uuid: Uuid,
    health: u32,
}

impl Baby {
    pub fn new(){}
}

impl Enemy for Baby {
    fn spawn(){}
}
