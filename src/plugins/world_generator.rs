use crate::AppState;
use bevy::prelude::Color;
use bevy::prelude::*;

// use bevy::text::cosmic_text::Transform;
use rand::{Rng, rng};

use noise::{BasicMulti, Perlin, utils::*};

use crate::plugins::game_menu::GameState;
use crate::plugins::game_menu::Hue;
use crate::plugins::ui::ResetMapEvent;

#[derive(Resource, Deref)]
pub struct Root(Entity);

fn get_hue_from_enum(hue: Hue) -> f32 {
    match hue {
        Hue::RED => 355.0,
        Hue::BLUE => 240.0,
        Hue::GREEN => 120.0,
        Hue::YELLOW => 60.0,
    }
}

fn generate_noise_map() -> NoiseMap {
    let mut rng = rng();
    let seed: u32 = rng.random();

    let basicmulti = BasicMulti::<Perlin>::new(seed);
    let basicmulti = PlaneMapBuilder::new(&basicmulti).build();

    return basicmulti;
}

fn get_color(val: f64, hue: Hue) -> Color {
    let value = get_hue_from_enum(hue);
    // saturation at 75, hue from enum, L from .1 to 1.
    let color_result = match val.abs() {
        v if v < 0.1 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.1)),
        v if v < 0.2 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.2)),
        v if v < 0.3 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.3)),
        v if v < 0.4 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.4)),
        v if v < 0.5 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.5)),
        v if v < 0.6 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.6)),
        v if v < 0.7 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.7)),
        v if v < 0.8 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.8)),
        v if v < 0.9 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.9)),
        v if v < 1.0 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 1.0)),
        _ => panic!("unexpected value"),
    };
    return color_result;
}

pub fn generate_world(
    mut commands: Commands,
    mut color: ResMut<Hue>,
    mut next_state: ResMut<NextState<GameState>>,
) {
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
                    let x = start_x + col_x as f32 * tile_size;
                    let y = start_y + col_y as f32 * tile_size;

                    parent.spawn((
                        Sprite {
                            color: get_color(val, color.clone()),
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

    //next_state.set(GameState::Finished);
}

pub fn cleanup(mut commands: Commands, root: Res<Root>) {
    commands.entity(**root).despawn();
}

pub fn reset(
    mut events: MessageReader<ResetMapEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        next_state.set(GameState::Build);
    }
}
