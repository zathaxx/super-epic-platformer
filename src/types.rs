/* Copyright 2022 Josias Allestad <me@josias.dev> and Jacob <zathaxx@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */
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
    pub fn right(&self) -> i32 {
        self.x + self.w
    }
    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }
    pub fn top(&self) -> i32 {
        self.y
    }
    pub fn left(&self) -> i32 {
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

#[derive(PartialEq)]
pub enum XSide {
    Right,
    Left,
    Center,
}

#[derive(PartialEq)]
pub enum YSide {
    Bottom,
    Center,
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
    pub velocity: Velocity,
}

impl Platform {
    pub fn new(pos: (i32, i32), size: (i32, i32)) -> Self {
        Self {
            hitbox: Hitbox {
                x: pos.0,
                y: pos.1,
                w: size.0,
                h: size.1,
            },
            color: Color::DARKGREEN,
            surface: Surface::new(),
            velocity: Velocity { x: 0, y: 0 },
        }
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn surface(mut self, surface: Surface) -> Self {
        self.surface = surface;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub speed: i32,
    pub transparent: bool,
    pub bounciness: f32,
    pub teleport: Option<(i32, i32)>,
    pub shifting: Option<((i32, i32), (i32, i32))>,
}

impl Surface {
    pub fn new() -> Self {
        Surface {
            speed: 5,
            transparent: false,
            bounciness: 0.3,
            teleport: None,
            shifting: None,
        }
    }
    pub fn _speed(mut self, speed: i32) -> Self {
        self.speed = speed;
        self
    }
    pub fn _transparent(mut self) -> Self {
        self.transparent = true;
        self
    }
    pub fn bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }
    pub fn teleport(mut self, teleport: (i32, i32)) -> Self {
        self.teleport = Some(teleport);
        self
    }
    pub fn shifting(mut self, locs: ((i32, i32), (i32, i32))) -> Self {
        self.shifting = Some(locs);
        self
    }
}
