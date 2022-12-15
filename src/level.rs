use crate::types::*;
use raylib::prelude::*;

fn default_character(grid_x: i32, grid_y: i32) -> Character {
    Character {
        hitbox: Hitbox {
            w: 30,
            h: 30,
            x: grid_x + 1000,
            y: grid_y,
        },
        color: Color::PURPLE,
        velocity: Velocity { x: 0, y: 0 },
    }
}

pub fn level_one() -> Level {
    let grid_x = 0;
    let grid_y = 0;
    let default = Surface {
        speed: 4,
        transparent: false,
    };
    let transparent = Surface {
        speed: 4,
        transparent: true,
    };

    Level {
        person: default_character(grid_x, grid_y),
        platforms: vec![
            Platform {
                hitbox: Hitbox {
                    x: grid_x,
                    y: grid_y,
                    w: 2000,
                    h: 1000,
                },
                surface: default.clone(),
                color: Color::BLACK,
            },
            Platform {
                hitbox: Hitbox {
                    x: grid_x + 950,
                    y: grid_y - 110,
                    w: 100,
                    h: 20,
                },
                surface: default.clone(),
                color: Color::BLACK,
            },
            Platform {
                hitbox: Hitbox {
                    x: grid_x + 950,
                    y: grid_y - 210,
                    w: 100,
                    h: 20,
                },
                surface: transparent.clone(),
                color: Color::YELLOW,
            },
            Platform {
                hitbox: Hitbox {
                    x: grid_x + 1150,
                    y: grid_y - 210,
                    w: 100,
                    h: 20,
                },
                surface: default.clone(),
                color: Color::BLACK,
            },
        ],
    }
}
