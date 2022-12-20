use crate::types::*;
use raylib::prelude::*;

fn default_character(grid_x: i32, grid_y: i32) -> Character {
    Character {
        hitbox: Hitbox {
            w: 90,
            h: 180,
            x: grid_x + 1000,
            y: grid_y - 60,
        },
        color: Color::PURPLE,
        velocity: Velocity { x: 0, y: 0 },
    }
}

pub fn level_one() -> Level {
    let grid_x = 0;
    let grid_y = 0;
    let default = Surface::new();
    let transparent = Surface::new().transparent();
    let bouncy = Surface::new().bounciness(0.75);
    let reset = Surface::new().teleport((grid_x + 1000, grid_y - 60));

    Level {
        person: default_character(grid_x, grid_y),
        platforms: vec![
            //Bottom platform
            Platform::new((grid_x, grid_y), (4000, 1000)),
            //Left wall
            Platform::new((grid_x, grid_y - 10000), (100, 10000)),
            //Right wall
            Platform::new((grid_x + 3900, grid_y - 10000), (100, 10000)),
            
            
            /*Platform::new((grid_x + 950, grid_y - 420), (200, 40)),
            Platform::new((grid_x + 850, grid_y - 420), (200, 40))
                .surface(transparent)
                .color(Color::YELLOW),
            Platform::new((grid_x + 1150, grid_y - 420), (200, 40)),
            Platform::new((grid_x + 1550, grid_y - 420), (200, 40))
                .surface(bouncy)
                .color(Color::GREEN),
            Platform::new((grid_x + 500, grid_y - 420), (20, 420)),
            Platform::new((grid_x + 200, grid_y - 200), (200, 40))
                .surface(bouncy)
                .color(Color::GREEN),
            Platform::new((grid_x + 1800, grid_y - 200), (200, 40))
                .surface(reset)
                .color(Color::BLUE),*/
        ],
    }
}
