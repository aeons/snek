use std::collections::HashMap;

use bevy::prelude::*;
use turborand::prelude::*;

use crate::components::Direction;
use crate::constants::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, SPRITE_SCALE};
use crate::resources::Random;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum Sprites {
    Food = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum SnakeSprites {
    Tail = 0,
    Middle,
    Head,
    CornerNorthWest = 7,
    CornerNorthEast,
    CornerSouthWest,
    CornerSouthEast,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum GrassSprites {
    Light = 4,
    Dark = 11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum GrassOverlaySprites {
    North = 5,
    East,
    West = 12,
    South,
}

fn sprite(
    sprite_index: usize,
    transform: Transform,
    texture_atlas: Handle<TextureAtlas>,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        texture_atlas,
        sprite: TextureAtlasSprite::new(sprite_index),
        transform,
        ..Default::default()
    }
}

pub fn spawn_snake(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    x: usize,
    y: usize,
    dir: Direction,
) {
    commands.spawn(sprite(
        SnakeSprites::Head as usize,
        dbg!(translate_xy(x, y, 1.))
            .with_scale(SPRITE_SCALE)
            .with_rotation(dir.as_rotation()),
        texture_atlas.clone(),
    ));

    let tail_transform = match dir {
        Direction::North => translate_xy(x, y - 1, 1.),
        Direction::South => translate_xy(x, y + 1, 1.),
        Direction::East => translate_xy(x - 1, y, 1.),
        Direction::West => translate_xy(x + 1, y, 1.),
    }
    .with_scale(SPRITE_SCALE)
    .with_rotation(dir.as_rotation());

    commands.spawn(sprite(
        SnakeSprites::Tail as usize,
        tail_transform,
        texture_atlas.clone(),
    ));
}

pub fn spawn_food(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    x: usize,
    y: usize,
) {
    commands.spawn(SpriteSheetBundle {
        texture_atlas,
        sprite: TextureAtlasSprite::new(Sprites::Food as usize),
        transform: translate_xy(x, y, 1.).with_scale(SPRITE_SCALE),
        ..Default::default()
    });
}

pub fn spawn_grid(
    commands: &mut Commands,
    rng: ResMut<Random>,
    texture_atlas: Handle<TextureAtlas>,
) {
    let grass_sprites = [GrassSprites::Light, GrassSprites::Dark];
    let mut cells = HashMap::with_capacity(GRID_WIDTH * GRID_HEIGHT);
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let sprite = rng
                .sample(&grass_sprites)
                .expect("grass sprite should be sampled");

            cells.insert((x, y), sprite);
        }
    }

    let overlay_sprite = |sprite: GrassOverlaySprites| SpriteSheetBundle {
        texture_atlas: texture_atlas.clone(),
        sprite: TextureAtlasSprite::new(sprite as usize),
        transform: Transform::from_xyz(0., 0., 0.1),
        ..Default::default()
    };

    for (&(x, y), &&sprite) in cells.iter() {
        let mut cell = commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(sprite as usize),
            transform: translate_xy(x, y, 0.).with_scale(SPRITE_SCALE),
            ..Default::default()
        });

        if sprite == GrassSprites::Light {
            cell.add_children(|parent| {
                if x > 0 && let Some(&&other) = cells.get(&(x - 1, y)) && other == GrassSprites::Dark {
                    parent.spawn(overlay_sprite(GrassOverlaySprites::West));
                }
                if x < GRID_WIDTH - 1 && let Some(&&other) = cells.get(&(x + 1, y)) && other == GrassSprites::Dark {
                    parent.spawn(overlay_sprite(GrassOverlaySprites::East));
                }
                if y > 0 && let Some(&&other) = cells.get(&(x , y - 1)) && other == GrassSprites::Dark {
                    parent.spawn(overlay_sprite(GrassOverlaySprites::South));
                }
                if y < GRID_WIDTH - 1 && let Some(&&other) = cells.get(&(x, y + 1)) && other == GrassSprites::Dark {
                    parent.spawn(overlay_sprite(GrassOverlaySprites::North));
                }
            });
        }
    }
}

fn translate_xy(x: usize, y: usize, z: f32) -> Transform {
    Transform::from_xyz(translate_x(x), translate_y(y), z)
}

fn translate_x(x: usize) -> f32 {
    let x = x as i32 - GRID_WIDTH as i32 / 2;
    (x as f32 + 0.5) * CELL_SIZE as f32
}

fn translate_y(y: usize) -> f32 {
    let y = y as i32 - GRID_HEIGHT as i32 / 2;
    (y as f32 + 0.5) * CELL_SIZE as f32
}
