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

        for platform in &mut *platforms {
            // Surface-specific behavior
            if platform.surface.transparent {
                continue;
            }
            let mut going_right = false;
            let mut going_down = false;
            if let Some(locs) = platform.surface.shifting {
                if locs.0 .0 != locs.1 .0 {
                    if platform.velocity.x >= 0 && platform.hitbox.x < locs.1 .0
                        || platform.hitbox.x < locs.0 .0
                    {
                        platform.velocity.x = 10;
                        going_right = true;
                    } else {
                        platform.velocity.x = -10;
                    }
                }

                if locs.0 .1 != locs.1 .1 {
                    if platform.velocity.y >= 0 && platform.hitbox.y < locs.1 .1
                        || platform.hitbox.y < locs.0 .1
                    {
                        platform.velocity.y = 10;
                        going_down = true;
                    } else {
                        platform.velocity.y = -10;
                    }
                }
            }
            platform.hitbox.x += platform.velocity.x;


            // Detect collisions
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

                    if let Some(locs) = platform.surface.shifting {
                        if locs.0 .0 != locs.1 .0 && person.velocity.x == 0 {
                            if going_right {
                                person.hitbox.x += 10;
                            } else {
                                person.hitbox.x -= 10;
                            }
                        }

                        if locs.0 .1 != locs.1 .1 && person.velocity.y == 0 {
                            if going_down {
                                person.hitbox.y += 10;
                            } else {
                                person.hitbox.y -= 10;
                            }
                        }
                    }
                } else if (side == Side::Top
                    || person.hitbox.y >= platform.hitbox.y + platform.hitbox.h)
                    && person.hitbox.left() < platform.hitbox.right()
                    && person.hitbox.right() > platform.hitbox.left()
                {
                    person.hitbox.y = platform.hitbox.y + platform.hitbox.h;
                    person.velocity.y = 0;
                } else if person.hitbox.top() < platform.hitbox.bottom()
                    && person.hitbox.bottom() > platform.hitbox.top()
                {
                    person.velocity.x = 0;
                    if side == Side::Right {
                        person.hitbox.x = platform.hitbox.left() - person.hitbox.w;
                    } else if side == Side::Left {
                        person.hitbox.x = platform.hitbox.right();
                    }
                }

            }

            // Separated from the equivalent for x for the engine's sake
            platform.hitbox.y += platform.velocity.y;
        }

        if !disable_gravity {
            person.hitbox.y += person.velocity.y * dt;
        }

        person.velocity.y += acceleration * dt;

        person.hitbox.x += person.velocity.x;

        let mut slowing_down = true;

        if rl.is_key_down(KeyboardKey::KEY_SPACE) && touching_ground {
            person.velocity.y += -30
        }
        if rl.is_key_down(KeyboardKey::KEY_F) {
            person.velocity.y = -30
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

        // Rendering preparation starts here.
        // TODO: consider separating into its own function.

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();

        let mut game_width = 0;
        for platform in &*platforms {
            if platform.hitbox.right() > game_width {
                game_width = platform.hitbox.right();
            }
        }

        let x_side = if person.hitbox.x <= screen_width / 2 {
            XSide::Left
        } else if person.hitbox.x >= game_width - screen_width / 2 {
            XSide::Right
        } else {
            XSide::Center
        };

        let bottom_offset = 50;
        let y_side = if person.hitbox.y - bottom_offset >= -(screen_height / 2) {
            YSide::Bottom
        } else {
            YSide::Center
        };

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);

        for platform in platforms {
            d.draw_rectangle(
                match x_side {
                    XSide::Left => platform.hitbox.x,
                    XSide::Right => platform.hitbox.x - (game_width - screen_width),
                    XSide::Center => platform.hitbox.x - person.hitbox.x + screen_width / 2,
                },
                match y_side {
                    YSide::Bottom => platform.hitbox.y + screen_height - bottom_offset,
                    YSide::Center => platform.hitbox.y - person.hitbox.y + screen_height / 2,
                },
                platform.hitbox.w,
                platform.hitbox.h,
                platform.color,
            );
        }

        d.draw_rectangle(
            match x_side {
                XSide::Left => person.hitbox.x,
                XSide::Right => person.hitbox.x - (game_width - screen_width),
                XSide::Center => screen_width / 2,
            },
            match y_side {
                YSide::Bottom => person.hitbox.y + screen_height - bottom_offset,
                YSide::Center => screen_height / 2,
            },
            person.hitbox.w,
            person.hitbox.h,
            person.color,
        );

        d.draw_text(
            &format!("x: {}, y: {}", person.hitbox.x, person.hitbox.y),
            10,
            10,
            40,
            Color::LIGHTGRAY,
        );
    }
}
