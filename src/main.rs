use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

pub mod common;
mod plugins;

use crate::common::AppState;
use crate::plugins::world_generator::{cleanup, generate_world, reset};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info"
                        .to_string(),
                    ..Default::default()
                }),
            plugins::camera::CameraPlugin,
            plugins::ui::UiPlugin,
        )) // prevents blurry sprites
        .insert_state::<AppState>(AppState::default())
        .add_systems(OnEnter(AppState::Build), generate_world)
        .add_systems(OnExit(AppState::Finished), cleanup)
        .add_systems(Update, reset.run_if(in_state(AppState::Finished)))
        .run();
}
