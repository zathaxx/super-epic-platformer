use crate::types::*;
use raylib::prelude::*;

fn default_character() -> Character {
    Character {
        hitbox: Hitbox {
            w: 30,
            h: 30,
            x: 1000,
            y: 1000,
        },
        color: Color::PURPLE,
        velocity: Velocity { x: 0, y: 0 },
    }
}

pub fn level_one() -> Level {
    Level {
        person: default_character(),
        platforms: vec![
            Platform {
                hitbox: Hitbox {
                    x: 950,
                    y: 340,
                    w: 100,
                    h: 20,
                },
                color: Color::BLACK,
            },
            Platform {
                hitbox: Hitbox {
                    x: 1150,
                    y: 240,
                    w: 100,
                    h: 20,
                },
                color: Color::BLACK,
            },
            Platform {
                hitbox: Hitbox {
                    x: 0,
                    y: 450,
                    w: 2000,
                    h: 1000,
                },
                color: Color::BLACK,
            },
        ],
    }
}
