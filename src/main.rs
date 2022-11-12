use raylib::prelude::*;

struct Character {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
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

    let mut person: Character = Character {
        x: 320,
        y: 0,
        width: 30,
        height: 30,
        color: Color::PURPLE,
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

    let mut velocity = 0;
    let acceleration = 1;
    let mut t = 0;
    let dt = 1;

    while !rl.window_should_close() {
        let mut bottom_left = 0;
        let location = person.height + person.y + (velocity * dt);
        let orig_vel = velocity;
        let mut touching_ground = rl.get_screen_height() - person.height == person.y;

        let mut near_collision = false;
        for platform in &platforms {
            if person.x + person.width > platform.x && person.x < platform.x + platform.width {
                if location > platform.y && location < platform.y + platform.height {
                    near_collision = true;
                    bottom_left = platform.y - person.height;
                    velocity = 0;
                    touching_ground = true;
                    break;
                } else if location - platform.height < platform.y + person.height
                    && !(location < platform.y + platform.height)
                {
                    velocity = 0;
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
                    velocity = orig_vel;
                } else if platform.x + platform.width == person.x {
                    move_left = false;
                    velocity = orig_vel;
                }
            }
        }

        if person.y < rl.get_screen_height() - person.height || velocity < 0 {
            if person.height + person.y + (velocity * dt) > rl.get_screen_height() {
                person.y = rl.get_screen_height() - person.height;
            } else if near_collision {
                person.y = bottom_left;
            } else {
                person.y += velocity * dt;
            }
            velocity += acceleration * dt;
            t = t + dt;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if touching_ground {
                velocity = -20
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
