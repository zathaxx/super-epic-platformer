use serde::{Deserialize, Serialize};

pub const level_one: &str = r#"
Level(
    initial_pos: (100, 100),
    platforms: [
        Platform(
            pos: (100, 90),
            size: (4, 2),
        ),
    ],
)
"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct Level {
    initial_pos: (i64, i64),
    platforms: Vec<Platform>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pos: (i64, i64),
    size: (i64, i64),
}

pub fn load(raw: &str) -> Level {
    let level: Level = ron::from_str(raw).unwrap();
    level
}
