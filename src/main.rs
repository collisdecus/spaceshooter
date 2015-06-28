extern crate piston_window;
extern crate rand;
extern crate find_folder;

mod util;
mod world;
mod entities;

use piston_window::*;
use world::*;

fn build_glyphs(window: &PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("font.ttf");
    let factory = window.factory.borrow().clone();
    Glyphs::new(font, factory).unwrap()
}

fn main() {
    let window: PistonWindow = 
        WindowSettings::new("Space Shooter", [640, 480])
        .exit_on_esc(true).into();

    let mut world = World::new();
    world.update_viewport(window.draw_size().width, window.draw_size().height);

    let mut glyphs = build_glyphs(&window);

    for e in window {
        e.draw_2d(|c, g| {
            clear([0.0; 4], g);

            for (r, p) in world.renderables() {
                polygon(r.color, &r.polygon, c.trans(p.x, p.y).transform, g);
            }

            let mut display_string = format!("Score: {}", world.score());
            if world.get_ship().is_destroyed() {
            	display_string = display_string + &" (You ship is gone. Press R to restart)".to_string();
            }

            text::Text::colored([0.0, 1.0, 0.0, 1.0], 16).draw(
                &display_string,
                &mut glyphs,
                &c.draw_state,
                c.trans(20.0, 24.0).transform, g
            );
        });

        e.update(|u| {
            world.run(u.dt)
        });

        {
            let mut button_handler = |b, enable| {
                {
                    let mut ship = world.get_ship();
                    match b {
                        Button::Keyboard(Key::Left) => ship.thrust_left(enable),
                        Button::Keyboard(Key::Right) => ship.thrust_right(enable),
                        Button::Keyboard(Key::Space) => ship.fire(enable),
                        _ => {}
                    }
                }

                if b == Button::Keyboard(Key::R) {
                    world.reset();
                }
            };

            e.press(|b| {
                button_handler(b, true)
            });

            e.release(|b| {
                button_handler(b, false)
            });
        }

        e.resize(|w, h| {
            world.update_viewport(w, h)
        });

        e.focus(|f| {
            world.pause(f)
        });
    }
}