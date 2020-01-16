use piston_window::{PistonWindow, WindowSettings};

const WINDOW_WIDTH : f64 = 1600.0;
const WINDOW_HEIGHT : f64 = 1000.0;

pub struct View {
    pub window : PistonWindow,
}

impl View {
    pub fn new() -> View {
        let window = WindowSettings::new("zombies", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .samples(4)
            .build()
            .unwrap();
        View {
            window
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}
