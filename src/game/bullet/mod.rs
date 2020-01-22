use gfx_device_gl::Texture;

pub struct Bullet {
    damage: u32,
    texture: Texture,
    velocity: f32,
}

enum BulletType {
}

impl Bullet {
    pub fn new(bullet_type: BulletType) -> Bullet {
        let damage: u32;
        let texture: Texture;
        let velocity: f32;

        match bullet_type {
            BulletType::Standard => {
                damage = 10;
                velocity = 2.0;
                texture =
            }
        }

        Bullet {
            damage,
            texture,
            velocity,
        }
    }
}
