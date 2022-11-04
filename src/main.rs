use std::thread::current;

use raylib::prelude::*;

struct Character {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
}
fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("BOUNCY BOUNCY").build();

    rl.set_target_fps(60);

    let mut person: Character = Character {
        x: 320,
        y: 0,
        width: 30,
        height: 30,
        color: Color::PURPLE,
    };

    let mut last_time = std::time::Instant::now();
    let mut velocity = 0;
    let acceleration = 1;

    let mut t = 0;
    let dt = 1;

    while !rl.window_should_close() {
        let current_time = std::time::Instant::now();

        if person.y < rl.get_screen_height() - person.height || velocity < 0 {
            if person.height + person.y + (velocity * dt) > rl.get_screen_height() {
                person.y = rl.get_screen_height() - person.height;
            } else {
                person.y += (velocity * dt);
            }
            velocity += acceleration * dt;
            t += dt;
        }

        last_time = current_time;

        let touching_ground = rl.get_screen_height() - person.height == person.y;

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if (touching_ground) {
                velocity = -20
            }
        }       
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            person.x += 5;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            person.x -= 5;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_rectangle(
            person.x,
            person.y,
            person.width,
            person.height,
            person.color,
        );
    }
}
