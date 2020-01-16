extern crate piston_window;

use zombies::game::world::World;
use zombies::game::view::View;

use piston_window::{Event, Events, EventSettings, Input, Loop};

fn main() {

    let game_world = World::default();
    let mut game_view = View::default();

    let mut events = Events::new(EventSettings::new());

    while let Some(event) = events.next(&mut game_view.window) {
        match event {
            Event::Input(input_event, _0) => {
                if let Input::Button(key) = input_event {
                    println!("{:#?}", key);
                }
            }
            Event::Loop(loop_event) => match loop_event {
//                Loop::Update(_) => game.update(),
                Loop::Render(_) => {
                    game_view.window.draw_2d(&event, |context, graphics, _| {
//                        let transform = context.transform.trans(0.0, 0.0);
//                        game.render(context, transform, graphics);
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
