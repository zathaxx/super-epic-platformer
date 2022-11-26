use raylib::prelude::*;

struct Velocity {
    x: i32,
    y: i32,
}

struct Hitbox {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Hitbox {
    fn collides(&self, b2: &Self) -> bool {
        self.x + self.w > b2.x
            && self.x < b2.x + b2.w
            && self.y + self.h > b2.y
            && self.y < b2.y + b2.h
    }
    fn touching(&self, b2: &Self) -> bool {
        self.x + self.w >= b2.x
            && self.x <= b2.x + b2.w
            && self.y + self.h >= b2.y
            && self.y <= b2.y + b2.h

    }
}

struct Character {
    hitbox: Hitbox,
    color: Color,
    velocity: Velocity,
}

struct Platform {
    hitbox: Hitbox,
    color: Color,
}

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
    ];

    let acceleration = 1;
    let mut t = 0;
    let dt = 1;

    while !rl.window_should_close() {
        let mut bottom_left = 0;
        let location = person.hitbox.h + person.hitbox.y + (person.velocity.y * dt);
        let orig_vel = person.velocity.y;
        let mut touching_ground = rl.get_screen_height() - person.hitbox.h == person.hitbox.y;

        let mut near_collision = false;
        for platform in &platforms {
            if person.hitbox.x + person.hitbox.w > platform.hitbox.x
                && person.hitbox.x < platform.hitbox.x + platform.hitbox.w
            {
                if location > platform.hitbox.y && location < platform.hitbox.y + platform.hitbox.h
                {
                    near_collision = true;
                    bottom_left = platform.hitbox.y - person.hitbox.h;
                    person.velocity.y = 0;
                    touching_ground = true;
                    break;
                } else if location - platform.hitbox.h < platform.hitbox.y + person.hitbox.h
                    && !(location < platform.hitbox.y + platform.hitbox.h)
                {
                    person.velocity.y = 0;
                    break;
                }
            }
        }

        let next_pos = Hitbox {
            x: person.hitbox.x + person.velocity.x,
            y: person.hitbox.y + person.velocity.y,
            ..person.hitbox
        };

        for platform in &platforms {
            if next_pos.collides(&platform.hitbox) {
                person.velocity.x = 0;
                //person.velocity.y = 0;
            }
        }

        if person.hitbox.y < rl.get_screen_height() - person.hitbox.h || person.velocity.y < 0 {
            if person.hitbox.h + person.hitbox.y + (person.velocity.y * dt) > rl.get_screen_height()
            {
                person.hitbox.y = rl.get_screen_height() - person.hitbox.h;
            } else if near_collision {
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
