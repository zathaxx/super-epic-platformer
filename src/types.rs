use raylib::prelude::*;

pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

pub struct Hitbox {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Hitbox {
    fn right(&self) -> i32 {
        self.x + self.w
    }
    fn bottom(&self) -> i32 {
        self.y + self.h
    }
    fn top(&self) -> i32 {
        self.y
    }
    fn left(&self) -> i32 {
        self.x
    }
    pub fn collides_with(&self, b2: &Self) -> bool {
        self.x + self.w > b2.x
            && self.x < b2.x + b2.w
            && self.y + self.h > b2.y
            && self.y < b2.y + b2.h
    }
    pub fn touches_side(&self, b2: &Self) -> Side {
        let bottom_diff = (self.bottom() - b2.top()).abs();
        let top_diff = (self.top() - b2.bottom()).abs();
        let right_diff = (self.right() - b2.left()).abs();
        let left_diff = (self.left() - b2.right()).abs();

        let mut side = Side::Bottom;
        let mut n = bottom_diff;
        if top_diff < n {
            side = Side::Top;
            n = top_diff;
        }
        if left_diff < n {
            side = Side::Left;
            n = left_diff;
        }
        if right_diff < n {
            side = Side::Right;
        }

        side
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

pub struct Level {
    pub person: Character,
    pub platforms: Vec<Platform>,
}

pub struct Character {
    pub hitbox: Hitbox,
    pub color: Color,
    pub velocity: Velocity,
}

pub struct Platform {
    pub hitbox: Hitbox,
    pub color: Color,
    pub surface: Surface,
}

#[derive(Clone)]
pub struct Surface {
    pub speed: i32,
    pub transparent: bool,
    pub bounciness: f32,
}

impl Surface {
    pub fn new() -> Self {
        Surface {
            speed: 4,
            transparent: false,
            bounciness: 1.0,
        }
    }
    pub fn speed(mut self, speed: i32) -> Self {
        self.speed = speed;
        self
    }
    pub fn transparent(mut self) -> Self {
        self.transparent = true;
        self
    }
    pub fn bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }
}
