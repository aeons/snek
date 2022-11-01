use std::collections::HashMap;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use turborand::prelude::*;

use crate::components::Direction;
use crate::constants::SPRITE_SIZE;
use crate::resources::Random;
use crate::spawn::{spawn_food, spawn_grid, spawn_snake};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    random: ResMut<Random>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(SPRITE_SIZE, SPRITE_SIZE),
        7,
        2,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());

    spawn_grid(&mut commands, random, texture_atlas_handle.clone());
    spawn_food(&mut commands, texture_atlas_handle.clone(), 1, 5);
    spawn_snake(
        &mut commands,
        texture_atlas_handle.clone(),
        5,
        10,
        Direction::North,
    );

    spawn_snake(
        &mut commands,
        texture_atlas_handle.clone(),
        10,
        10,
        Direction::South,
    );

    spawn_snake(
        &mut commands,
        texture_atlas_handle.clone(),
        15,
        10,
        Direction::East,
    );

    spawn_snake(
        &mut commands,
        texture_atlas_handle.clone(),
        20,
        10,
        Direction::West,
    );
}
