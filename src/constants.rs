use bevy::prelude::Vec3;

pub const SPRITE_SIZE: f32 = 384.;
pub const CELL_SIZE: usize = 48;
pub const SPRITE_SCALE: Vec3 = Vec3::splat(CELL_SIZE as f32 / SPRITE_SIZE);
pub const GRID_WIDTH: usize = 40;
pub const GRID_HEIGHT: usize = 20;
