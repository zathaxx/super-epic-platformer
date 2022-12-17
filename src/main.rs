use raylib::prelude::*;

mod level;
mod types;

use types::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(1080, 720).title("BOUNCY BOUNCY").build();

    rl.set_target_fps(60);

    let mut level = level::level_one();

    let acceleration = 1;
    let dt = 1;

    while !rl.window_should_close() {
        let platforms = &mut level.platforms;
        let person = &mut level.person;

        let mut bottom_left = 0;
        let next_y = person.hitbox.y + person.hitbox.h + (person.velocity.y * dt);
        let mut touching_ground = false;
        let mut near_collision = false;

        let next_pos = Hitbox {
            x: person.hitbox.x + person.velocity.x,
            y: person.hitbox.y + person.velocity.y,
            ..person.hitbox
        };

        let mut surface_speed = 4;

        for platform in &*platforms {
            if platform.surface.transparent {
                continue;
            }
            if next_pos.collides(&platform.hitbox) {
                if person.velocity.y > 0 {
                    near_collision = true;
                    touching_ground = true;
                    bottom_left = platform.hitbox.y - person.hitbox.h;
                    surface_speed = platform.surface.speed;

                    if platform.surface.bouncy {
                        person.velocity.y = -person.velocity.y * 3 / 4;
                    } else {
                        person.velocity.y = 0;
                    }
                } else if person.velocity.y < 0 {
                    person.velocity.y = 0;
                } else {
                    person.velocity.x = 0;
                }
            }
        }

        if near_collision {
            person.hitbox.y = bottom_left;
        } else {
            person.hitbox.y += person.velocity.y * dt;
        }
        person.velocity.y += acceleration * dt;

        person.hitbox.x += person.velocity.x;

        let mut slowing_down = true;

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && touching_ground {
            person.velocity.y = -25
        }
        if rl.is_key_down(KeyboardKey::KEY_F) {
            person.velocity.y = -10
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            slowing_down = false;
            if person.velocity.x < 4 * surface_speed {
                person.velocity.x += 1 * surface_speed;
            } else {
                person.velocity.x = 4 * surface_speed;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            slowing_down = false;
            if person.velocity.x > -4 * surface_speed {
                person.velocity.x -= 1 * surface_speed;
            } else {
                person.velocity.x = -4 * surface_speed;
            }
        }

        if slowing_down {
            if person.velocity.x > 0 {
                person.velocity.x -= 1 * surface_speed;
            }
            if person.velocity.x < 0 {
                person.velocity.x += 1 * surface_speed;
            }
        }

        let width = rl.get_screen_width();
        let height = rl.get_screen_height();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_rectangle(
            width / 2,
            height / 2,
            person.hitbox.w,
            person.hitbox.h,
            person.color,
        );

        for platform in platforms {
            d.draw_rectangle(
                platform.hitbox.x - person.hitbox.x + width / 2,
                platform.hitbox.y - person.hitbox.y + height / 2,
                platform.hitbox.w,
                platform.hitbox.h,
                platform.color,
            );
        }

        d.draw_text(
            &format!("x: {}, y: {}", person.hitbox.x, person.hitbox.y),
            10,
            10,
            40,
            Color::LIGHTGRAY,
        );
    }
}
