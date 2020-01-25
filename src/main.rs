extern crate graphics;
extern crate piston_window;
extern crate vecmath;

use zombies::game::view::View;
use zombies::game::world::World;

use input::RenderEvent;
use piston_window::{Event, EventSettings, Events, Input, Loop};

fn main() {
    // Currently this needs to be instantiated before the world
    //      because the open_gl initiation occurs in it
    let mut game_view = View::default();
    let mut game_world = World::new();
    let mut events = Events::new(EventSettings::new());

    while let Some(event) = events.next(&mut game_view.window) {
        match event {
            Event::Input(input_event, _timestamp) => match input_event {
                Input::Button(key) => game_world.handle_button_event(key),
                Input::Move(motion) => game_world.handle_mouse(motion),
                _ => {}
            },
            Event::Loop(loop_event) => match loop_event {
                Loop::Update(_) => game_world.update(),
                Loop::Render(_) => {
                    if let Some(args) = event.render_args() {
                        game_view
                            .gl_graphics
                            .draw(args.viewport(), |context, graphics| {
                                let transform = context.transform;
                                game_world.render(context, transform, graphics);
                            })
                    }
                    //                    game_view.window.draw_2d(&event, |context, graphics, _| {
                    //                        game_world.render(context, context.transform, graphics);
                    //                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
