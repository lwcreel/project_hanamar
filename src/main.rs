use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

pub mod common;
mod plugins;

use crate::common::AppState;
use crate::plugins::game_menu::{DisplayQuality, GameState, Volume, game, menu, setup, splash};
use crate::plugins::world_generator::{cleanup, generate_world, reset};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info"
                    .to_string(),
                ..Default::default()
            }),)) // prevents blurry sprites
        .init_state::<GameState>()
        .insert_resource(DisplayQuality::BLUE)
        .insert_resource(Volume(7))
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}
