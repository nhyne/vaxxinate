use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::{PistonWindow, WindowSettings};

const WINDOW_WIDTH: f64 = 1600.0;
const WINDOW_HEIGHT: f64 = 900.0;

pub struct View {
    pub window: PistonWindow,
    pub gl_graphics: GlGraphics,
}

impl View {
    pub fn new() -> View {
        let open_gl = OpenGL::V3_2;
        let window = WindowSettings::new("Zombies", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .graphics_api(open_gl)
            .build()
            .unwrap();
        let mut gl_graphics = GlGraphics::new(open_gl);

        View {
            window,
            gl_graphics,
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}
