use crate::config::settings::Settings;
use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::{PistonWindow, WindowSettings};

pub struct View {
    pub window: PistonWindow,
    pub gl_graphics: GlGraphics,
}

impl View {
    pub fn new(config: &Settings) -> View {
        let open_gl = OpenGL::V3_2;
        let window = WindowSettings::new("Zombies", [config.window.width, config.window.height])
            .exit_on_esc(true)
            .graphics_api(open_gl)
            .build()
            .unwrap();
        let gl_graphics = GlGraphics::new(open_gl);

        View {
            window,
            gl_graphics,
        }
    }
}
