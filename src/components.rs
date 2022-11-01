use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn as_rotation(&self) -> Quat {
        let angle = match self {
            Direction::North => FRAC_PI_2,
            Direction::South => 3. * FRAC_PI_2,
            Direction::East => 0.,
            Direction::West => PI,
        };
        Quat::from_rotation_z(angle)
    }
}
