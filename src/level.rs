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

    let mut platforms = vec![
        // Bottom platform
        Platform::new((grid_x, grid_y), (4000, 1000)),
        // Left wall
        Platform::new((grid_x, grid_y - 10000), (100, 10000)),
        // Right wall
        Platform::new((grid_x + 3900, grid_y - 10000), (100, 10000)),
    ];

    {
        // Jumping staircase
        let stair = (grid_x + 1200, grid_y - 500);
        let stairs = vec![
            Platform::new((stair.0, stair.1), (300, 75)),
            Platform::new((stair.0 + 800, stair.1 - 600), (300, 75)),
            Platform::new((stair.0 + 1600, stair.1 - 1200), (300, 75)),
        ];

        platforms.extend(stairs);
    }

    {
        // Row 2
        let row_start = (grid_x + 100, grid_y - 2000);
        let row = vec![
            Platform::new((row_start.0, row_start.1), (2000, 75))
                .color(Color::RED)
                .surface(reset),
            Platform::new((row_start.0 + 1925, row_start.1 - 300), (75, 300)),
            Platform::new((row_start.0 + 475, row_start.1 - 300), (75, 300)),
            Platform::new((grid_y + 100, grid_y - 2825), (50, 40)),
            Platform::new((grid_y + 100, grid_y - 3125), (50, 40)),
            Platform::new((grid_y + 100, grid_y - 3525), (50, 40)),
        ];

        platforms.extend(row);
    }

    Level {
        person: default_character(grid_x, grid_y),
        platforms,
    }
}
