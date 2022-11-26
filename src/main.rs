use raylib::prelude::*;

mod level;
mod types;

use types::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("BOUNCY BOUNCY").build();

    rl.set_target_fps(60);

    let mut person = Character {
        hitbox: Hitbox {
            x: 320,
            y: 0,
            w: 30,
            h: 30,
        },
        color: Color::PURPLE,
        velocity: Velocity { x: 0, y: 0 },
    };

    let platforms = vec![
        Platform {
            hitbox: Hitbox {
                x: 150,
                y: 340,
                w: 100,
                h: 20,
            },
            color: Color::BLACK,
        },
        Platform {
            hitbox: Hitbox {
                x: 350,
                y: 240,
                w: 100,
                h: 20,
            },
            color: Color::BLACK,
        },
        
        Platform {
            hitbox: Hitbox {
                x: 0,
                y: rl.get_screen_height() - 30,
                w: rl.get_screen_width(),
                h: 100,
            },
            color: Color::BLACK,
        },
    ];

    let acceleration = 1;
    let mut t = 0;
    let dt = 1;

    while !rl.window_should_close() {
        let mut bottom_left = 0;
        let next_loc = person.hitbox.y + person.hitbox.h + (person.velocity.y * dt);
        let mut touching_ground = false;
        let mut near_collision = false;

        let next_pos = Hitbox {
            x: person.hitbox.x + person.velocity.x,
            y: person.hitbox.y + person.velocity.y,
            ..person.hitbox
        };

        println!("{}", person.velocity.y);

        for platform in &platforms {
            // Checks if the Character is aligned with the current platform on the x-axis
            if person.hitbox.x + person.hitbox.w > platform.hitbox.x
                && person.hitbox.x < platform.hitbox.x + platform.hitbox.w
            {
                // Checks if the Character is about to hit the surface of the platform
                if next_loc > platform.hitbox.y && next_loc < platform.hitbox.y + platform.hitbox.h
                {
                    near_collision = true;
                    touching_ground = true;
                    bottom_left = platform.hitbox.y - person.hitbox.h;
                    person.velocity.y = 0;
                    break;
                } else if next_loc - platform.hitbox.h < platform.hitbox.y + person.hitbox.h
                    && !(next_loc < platform.hitbox.y + platform.hitbox.h)
                {
                    person.velocity.y = 0;
                    break;
                }
            }

            if next_pos.collides(&platform.hitbox) {
                person.velocity.x = 0;
            }
        }

        if person.hitbox.y < rl.get_screen_height() - person.hitbox.h + 100 || person.velocity.y < 0 {
            if near_collision {
                person.hitbox.y = bottom_left;
            } else {
                person.hitbox.y += person.velocity.y * dt;
            }
            person.velocity.y += acceleration * dt;
            t = t + dt;
        }

        person.hitbox.x += person.velocity.x;

        let mut slowing_down = true;

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if touching_ground {
                person.velocity.y = -20
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            slowing_down = false;
            if person.velocity.x < 4 {
                person.velocity.x += 1;
            } else {
                person.velocity.x = 4;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            slowing_down = false;
            if person.velocity.x > -4 {
                person.velocity.x -= 1;
            } else {
                person.velocity.x = -4;
            }
        }

        if slowing_down {
            if (person.velocity.x > 0) {
                person.velocity.x -= 1;
            }
            if (person.velocity.x < 0) {
                person.velocity.x += 1;
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_rectangle(
            person.hitbox.x,
            person.hitbox.y,
            person.hitbox.w,
            person.hitbox.h,
            person.color,
        );

        for platform in &platforms {
            d.draw_rectangle(
                platform.hitbox.x,
                platform.hitbox.y,
                platform.hitbox.w,
                platform.hitbox.h,
                platform.color,
            );
        }
    }
}
