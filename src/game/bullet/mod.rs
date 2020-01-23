use gfx_device_gl::Texture;
use std::rc::Rc;

// This system can probably be a lot more efficient.
pub struct Bullet {
    damage: u32,
    velocity: f32,
    texture: Rc<Texture>,
}

impl Bullet {
    pub fn new(damage: u32, velocity: f32, texture: Rc<Texture>) -> Bullet {
        Bullet {
            damage,
            velocity,
            texture,
        }
    }
}
