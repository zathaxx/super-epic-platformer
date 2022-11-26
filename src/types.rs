use raylib::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub fn collides(&self, b2: &Self) -> bool {
        self.x + self.w > b2.x
            && self.x < b2.x + b2.w
            && self.y + self.h > b2.y
            && self.y < b2.y + b2.h
    }
    pub fn touching(&self, b2: &Self) -> bool {
        self.x + self.w >= b2.x
            && self.x <= b2.x + b2.w
            && self.y + self.h >= b2.y
            && self.y <= b2.y + b2.h
    }
}

pub struct Character {
    pub hitbox: Hitbox,
    pub color: Color,
    pub velocity: Velocity,
}

pub struct Platform {
    pub hitbox: Hitbox,
    pub color: Color,
}
