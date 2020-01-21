use crate::game::projectile::bullet::Bullet;
use crate::game::renderable::Renderable;
use graphics::Context;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;
use nphysics2d::object::{DefaultBodySet, DefaultBodyHandle, DefaultColliderHandle};

pub struct Standard {
    damage: u32,
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
}

impl Bullet for Standard {
    fn damage(&self) {

    }

    fn should_drop(&self) -> bool {
        false
    }
}

impl Renderable for Standard {
     fn render(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut GlGraphics,
        world: &DefaultBodySet<f64>,
    ) {
        if let Some(body) = world.rigid_body(self.body_handle) {
            // Just want to draw as cube for now or should I use another image?
        }
    }
}
