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

    let platforms = vec![Platform {
        x: 150,
        y: 340,
        width: 100,
        height: 20,
        color: Color::BLACK,
    }, Platform {
        x: 350,
        y: 340,
        width: 100,
        height: 20,
        color: Color::BLACK, 
    }];

    let mut velocity = 0;
    let acceleration = 1;
    let mut t = 0;
    let dt = 1;

    while !rl.window_should_close() {
        
        let mut modifier = 0;
        let location = person.height + person.y + (velocity * dt);

        let mut near_collision = false;
        for platform in &platforms {
            if person.x + person.width >= platform.x && person.x <= platform.x + platform.width {
                if location > platform.y && location < platform.y + platform.height && velocity > 0
                {
                    near_collision = true;
                    modifier = platform.y - person.height;
                    velocity = 0;
                    break;
                } else if location - platform.height < platform.y + person.height
                    && !(location < platform.y + platform.height)
                {
                    velocity = 0;
                    break;
                }
            }
        }

        if person.y < rl.get_screen_height() - person.height || velocity < 0 {
            if person.height + person.y + (velocity * dt) > rl.get_screen_height() {
                person.y = rl.get_screen_height() - person.height;
            } else if near_collision {
                person.y = modifier;
            } else {
                person.y += velocity * dt;
            }
            velocity += acceleration * dt;
            t = t + dt;
        }

        let touching_ground = rl.get_screen_height() - person.height == person.y;

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if touching_ground {
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

        d.draw_rectangle(
            platforms[0].x,
            platforms[0].y,
            platforms[0].width,
            platforms[0].height,
            platforms[0].color,
        );

        d.draw_rectangle(
            platforms[1].x,
            platforms[1].y,
            platforms[1].width,
            platforms[1].height,
            platforms[1].color,
        );
    }
}
