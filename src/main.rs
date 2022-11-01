#![feature(let_chains)]

mod components;
mod constants;
mod resources;
mod spawn;
mod systems;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use constants::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use resources::Random;

use crate::systems::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(12, 4, 4)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "snek".to_string(),
                        width: (GRID_WIDTH * CELL_SIZE) as f32,
                        height: (GRID_HEIGHT * CELL_SIZE) as f32,
                        resizable: false,
                        position: WindowPosition::Centered,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugin(FrameTimeDiagnosticsPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .init_resource::<Random>()
        .add_startup_system(setup)
        .run()
}
