use crate::types::*;
use raylib::prelude::*;

fn default_character(grid_x: i32, grid_y: i32) -> Character {
    Character {
        hitbox: Hitbox {
            w: 90,
            h: 180,
            x: grid_x + 1000,
            y: grid_y - 90,
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
    let bouncy = Surface::new().bounciness(0.5);
    let reset = Surface::new().teleport((grid_x + 1000, grid_y - 90));
    let shifting_reset = Surface::new()
        .shifting(((grid_x + 100, grid_y), (grid_x + 3800, grid_y)))
        .teleport((grid_x + 1000, grid_y - 180));
    let row6_shifting_reset = Surface::new()
    .shifting(((grid_x + 900, grid_y - 9500), (grid_x + 2000, grid_y - 9500)))
    .teleport((grid_x + 1000, grid_y - 180));

    let mut platforms = vec![
        // Bottom platform
        Platform::new((grid_x, grid_y), (4000, 1000)),
        // Left wall
        Platform::new((grid_x, grid_y - 11000), (100, 11000)),
        // Right wall
        Platform::new((grid_x + 3900, grid_y - 11000), (100, 11000)),
    ];

    platforms.push(
        Platform::new((grid_x + 1500, grid_y), (100, 100))
            .surface(shifting_reset)
            .color(Color::RED),
    );

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

    {
        // Row 3
        let row_start = (grid_x + 1050, grid_y - 3525);
        let row = vec![
            Platform::new((row_start.0 + 500, row_start.1), (300, 75)),
            Platform::new((row_start.0 + 800, row_start.1 - 750), (300, 75))
                .color(Color::RED)
                .surface(reset),
            Platform::new((row_start.0 + 800, row_start.1 - 675), (300, 750)),
            Platform::new((row_start.0 + 800, row_start.1 - 1100), (300, 75)),
            Platform::new((row_start.0 + 1100, row_start.1), (1750, 75))
                .color(Color::BLUE)
                .surface(bouncy),
        ];

        platforms.extend(row);
    }

    {
        // Row 4
        let row_start = (grid_x + 3750, grid_y - 5200);
        let row = vec![
            Platform::new((row_start.0, row_start.1), (150, 75)),
            Platform::new((row_start.0 - 1000, row_start.1 - 600), (150, 75)),
            Platform::new((row_start.0, row_start.1 - 1000), (250, 75))
                .color(Color::ORANGE)
                .surface(Surface::new().shifting((
                    (row_start.0 - 3500, row_start.1 - 1000),
                    (row_start.0 - 1500, row_start.1 - 1000),
                ))),
            Platform::new((100, row_start.1 - 1300), (75, 75))
                .color(Color::BLUE)
                .surface(bouncy),
        ];

        platforms.extend(row);
    }

    {
        // Row 5
        let row_start = (grid_x + 500, grid_y - 7500);
        let row = vec![
            Platform::new((row_start.0, row_start.1), (250, 75))
                .color(Color::ORANGE)
                .surface(Surface::new().shifting((
                    (row_start.0, row_start.1),
                    (row_start.0 + 2500, row_start.1),
                ))),
            Platform::new((row_start.0 + 800, row_start.1 - 75), (75, 75))
                .color(Color::RED)
                .surface(reset),
            Platform::new((row_start.0 + 1800, row_start.1 - 75), (75, 75))
                .color(Color::RED)
                .surface(reset),
            Platform::new((row_start.0 + 3250, row_start.1), (150, 75)),
        ];
        platforms.extend(row);
    }

    {
        // Row 5
        let row_start = (grid_x + 3200, grid_y - 8300);
        let row = vec![
            Platform::new((2500, -7000), (1400, 75))
                .color(Color::RED)
                .surface(reset),
            Platform::new((row_start.0, row_start.1), (150, 75)),
            Platform::new((row_start.0 - 200, row_start.1), (75, 75)).surface(
                Surface::new().shifting((
                    (row_start.0 - 200, row_start.1 - 1000),
                    (row_start.0 - 200, row_start.1),
                )),
            ),
        ];
        platforms.extend(row);
    }


    {
        // Row 6
        let row_start = (grid_x + 900, grid_y - 9500);
        let row = vec![
            Platform::new((row_start.0, row_start.1), (1250, 75)),
            Platform::new((row_start.0, row_start.1), (150, 75))
            .color(Color::RED)
            .surface(row6_shifting_reset),
            Platform::new((row_start.0 + 500, row_start.1- 1250), (90, 250))
            .color(Color::RED)
            .surface(
                Surface::new().shifting((
                    (row_start.0 + 500, row_start.1 - 1250),
                    (row_start.0 + 500, row_start.1 - 250),
                ))
                .teleport((grid_x + 1000, grid_y - 180)),
            ),
            Platform::new((row_start.0 - 700, row_start.1- 850), (150, 75))
                .color(Color::BLUE)
                .surface(bouncy),
                
        ];
        platforms.extend(row);
    }

    {
        // Final Row
        let row_start = (grid_x + 1900, grid_y - 11000);
        let row = vec![
            Platform::new((row_start.0, row_start.1), (2100, 75)),                
        ];
        platforms.extend(row);
    }

    Level {
        person: default_character(grid_x, grid_y),
        platforms,
    }
}
