use raylib::prelude::*;

struct Velocity {
    x: i32,
    y: i32,
}

struct Character {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
    velocity: Velocity,
}

struct Platform {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("BOUNCY BOUNCY").build();

    rl.set_target_fps(60);

    let mut person = Character {
        x: 320,
        y: 0,
        width: 30,
        height: 30,
        color: Color::PURPLE,
        velocity: Velocity {
            x: 0,
            y: 0,
        },
    };

    let platforms = vec![
        Platform {
            x: 150,
            y: 340,
            width: 100,
            height: 20,
            color: Color::BLACK,
        },
        Platform {
            x: 350,
            y: 240,
            width: 100,
            height: 20,
            color: Color::BLACK,
        },
    ];

    let acceleration = 1;
    let mut t = 0;
    let dt = 1;

    while !rl.window_should_close() {
        let mut bottom_left = 0;
        let location = person.height + person.y + (person.velocity.y * dt);
        let orig_vel = person.velocity.y;
        let mut touching_ground = rl.get_screen_height() - person.height == person.y;

        let mut near_collision = false;
        for platform in &platforms {
            if person.x + person.width > platform.x && person.x < platform.x + platform.width {
                if location > platform.y && location < platform.y + platform.height {
                    near_collision = true;
                    bottom_left = platform.y - person.height;
                    person.velocity.y = 0;
                    touching_ground = true;
                    break;
                } else if location - platform.height < platform.y + person.height
                    && !(location < platform.y + platform.height)
                {
                    person.velocity.y = 0;
                    break;
                }
            }
        }

        let mut move_left = true;
        let mut move_right = true;
        for platform in &platforms {
            if platform.y < person.y + person.height && platform.y + platform.height > person.y {
                if platform.x == person.x + person.width {
                    move_right = false;
                    person.velocity.y = orig_vel;
                } else if platform.x + platform.width == person.x {
                    move_left = false;
                    person.velocity.y = orig_vel;
                }
            }
        }

        if person.y < rl.get_screen_height() - person.height || person.velocity.y < 0 {
            if person.height + person.y + (person.velocity.y * dt) > rl.get_screen_height() {
                person.y = rl.get_screen_height() - person.height;
            } else if near_collision {
                person.y = bottom_left;
            } else {
                person.y += person.velocity.y * dt;
            }
            person.velocity.y += acceleration * dt;
            t = t + dt;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if touching_ground {
                person.velocity.y = -20
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            if move_right {
                person.x += 5;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            if move_left {
                person.x -= 5;
            }
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

        for platform in &platforms {
            d.draw_rectangle(
                platform.x,
                platform.y,
                platform.width,
                platform.height,
                platform.color,
            );
        }
    }
}
