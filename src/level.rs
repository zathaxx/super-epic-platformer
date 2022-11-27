use raylib::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types;

pub const level_one: &str = r#"
Level(
    initial_pos: (320, 0),
    platforms: [
        Platform(
            pos: (150, 340),
            size: (100, 20),
        ),
        Platform(
            pos: (350, 240),
            size: (100, 20),
        ),
    ],
)
"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct Level {
    initial_pos: (i32, i32),
    platforms: Vec<Platform>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pos: (i32, i32),
    size: (i32, i32),
}

pub fn load(raw: &str) -> types::Level {
    let level: Level = ron::from_str(raw).unwrap();

    let mut platforms = vec![];

    for platform in level.platforms {
        platforms.push(types::Platform {
            hitbox: types::Hitbox {
                x: platform.pos.0,
                y: platform.pos.1,
                w: platform.size.0,
                h: platform.size.1,
            },
            color: Color::BLACK,
        })
    }

    types::Level {
        person: types::Character {
            hitbox: types::Hitbox {
                x: level.initial_pos.0,
                y: level.initial_pos.1,
                w: 30,
                h: 30,
            },
            color: Color::PURPLE,
            velocity: types::Velocity {
                x: 0,
                y: 0,
            },
        },
        platforms,
    }
}
