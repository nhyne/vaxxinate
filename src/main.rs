extern crate piston_window;
extern crate vecmath;

use zombies::game::view::View;
use zombies::game::world::World;

use piston_window::{Event, EventSettings, Events, Input, Loop};
use vecmath::mat2x3_add;

fn main() {
    let mut game_world = World::default();
    let mut game_view = View::default();
    let mut events = Events::new(EventSettings::new());
    let mut frame = 0.0;

   while let Some(event) = events.next(&mut game_view.window) {
        match event {
            Event::Input(input_event, _0) => match input_event {
                Input::Button(key) => game_world.handle_key_press(key),
                Input::Move(motion) => game_world.handle_mouse(motion),
                _ => {}
            },
            Event::Loop(loop_event) => match loop_event {
                //                Loop::Update(_) => game.update(),
                Loop::Render(_) => {
                    game_view.window.draw_2d(&event, |context, graphics, _| {
                        let id_mat = vecmath::mat2x3_id();
                        let rotation_matrix = vecmath::mat2x3_sub(id_mat,piston_window::math::rotate_radians(frame));
                        println!("Rotation Matrix: {:#?}", rotation_matrix);
                        let transform = context.transform;
                        println!("Initial Transform: {:#?}", transform);
                        let something = mat2x3_add(rotation_matrix, transform);
                        println!("Final Matrix: {:#?}", something);
                        game_world.render(context, something, graphics);
                    });
                }
                _ => {}
            },
            _ => {}
        }
       frame = frame + 0.000001;
    }
}
