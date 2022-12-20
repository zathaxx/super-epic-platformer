use raylib::prelude::*;

mod level;
mod types;

use types::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1080, 720)
        .resizable()
        .title("BOUNCY BOUNCY")
        .build();

    rl.set_target_fps(60);

    let mut level = level::level_one();

    let acceleration = 1;
    let dt = 1;

    while !rl.window_should_close() {
        let platforms = &mut level.platforms;
        let person = &mut level.person;
        let mut touching_ground = false;
        let mut disable_gravity = false;

        let next_pos = Hitbox {
            x: person.hitbox.x + person.velocity.x,
            y: person.hitbox.y + person.velocity.y,
            ..person.hitbox
        };

        let mut surface_speed = 5;

        for platform in &*platforms {
            if platform.surface.transparent {
                continue;
            }
            if next_pos.collides_with(&platform.hitbox) {
                let side = next_pos.touches_side(&platform.hitbox);
                if (side == Side::Bottom || person.hitbox.y + person.hitbox.h <= platform.hitbox.y)
                    && person.hitbox.left() < platform.hitbox.right()
                    && person.hitbox.right() > platform.hitbox.left()
                {
                    disable_gravity = true;
                    touching_ground = true;
                    person.hitbox.y = platform.hitbox.y - person.hitbox.h;
                    surface_speed = platform.surface.speed;
                    person.velocity.y =
                        (-person.velocity.y as f32 * platform.surface.bounciness) as i32;
                    if let Some(teleport) = platform.surface.teleport {
                        person.hitbox.y = teleport.1;
                        person.hitbox.x = teleport.0;
                    }
                } else if (side == Side::Top
                    || person.hitbox.y >= platform.hitbox.y + platform.hitbox.h)
                    && person.hitbox.left() < platform.hitbox.right()
                    && person.hitbox.right() > platform.hitbox.left()
                {
                    person.hitbox.y = platform.hitbox.y + platform.hitbox.h;
                    person.velocity.y = 0;
                } else {
                    person.velocity.x = 0;
                }
            }
        }

        if !disable_gravity {
            person.hitbox.y += person.velocity.y * dt;
        }

        person.velocity.y += acceleration * dt;

        person.hitbox.x += person.velocity.x;

        let mut slowing_down = true;

        if rl.is_key_down(KeyboardKey::KEY_SPACE) && touching_ground {
            person.velocity.y = -35
        }
        if rl.is_key_down(KeyboardKey::KEY_F) {
            person.velocity.y = -20
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
        let mouse_pos = (rl.get_mouse_x() + person.hitbox.x - width / 2, rl.get_mouse_y() + person.hitbox.y - height / 2);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::SKYBLUE);

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
        d.draw_text(
            &format!("x: {}, y: {}", mouse_pos.0, mouse_pos.1),
            500,
            10,
            40,
            Color::LIGHTGRAY,
        );


        d.draw_rectangle(
            width / 2,
            height / 2,
            person.hitbox.w,
            person.hitbox.h,
            person.color,
        );
    }
}
