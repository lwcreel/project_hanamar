//! In this example we generate a new texture atlas (sprite sheet) from a folder containing
//! individual sprites.

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

// use bevy::text::cosmic_text::Transform;
use rand::{Rng, rng};

use noise::{BasicMulti, Perlin, utils::*};

pub mod common;
mod plugins;

use crate::common::AppState;
use plugins::ui::ResetMapEvent;

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

fn generate_noise_map() -> NoiseMap {
    let mut rng = rng();
    let seed: u32 = rng.random();

    let basicmulti = BasicMulti::<Perlin>::new(seed);
    let basicmulti = PlaneMapBuilder::new(&basicmulti).build();

    return basicmulti;
}

fn get_color(val: f64) -> bevy::prelude::Color {
    bevy::prelude::Color::hsl(120., 1., val as f32)
}

#[derive(Resource, Deref)]
struct Root(Entity);

fn generate_world(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let map = generate_noise_map();
    let (grid_width, grid_height) = map.size();
    debug!("Map size: {}x{}", grid_width, grid_height);

    let tile_size = 32_f32;

    let start_x = -(grid_width as f32) * tile_size / 2.0;
    let start_y = -(grid_height as f32) * tile_size / 2.0;

    let root = commands
        .spawn((Transform::default(), Visibility::default()))
        .with_children(|parent| {
            for col_x in 0..grid_width {
                for col_y in 0..grid_height {
                    let val = map.get_value(col_x, col_y);
                    // if val > 0.8_f64 {
                    // debug!("Value for {}:{} = {}", col_x, col_y, val);
                    // }
                    let x = start_x + col_x as f32 * tile_size;
                    let y = start_y + col_y as f32 * tile_size;

                    parent.spawn((
                        Sprite {
                            color: get_color(val),
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(x, y, 0.)),
                    ));
                }
            }
        })
        .id();

    commands.insert_resource(Root(root));

    next_state.set(AppState::Finished);
}

fn cleanup(mut commands: Commands, root: Res<Root>) {
    commands.entity(**root).despawn();
}

fn reset(mut events: EventReader<ResetMapEvent>, mut next_state: ResMut<NextState<AppState>>) {
    for _ in events.read() {
        next_state.set(AppState::Build);
    }
}
