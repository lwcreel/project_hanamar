use bevy::prelude::Color;
use bevy::prelude::Srgba;
use bevy::prelude::*;

// use bevy::text::cosmic_text::Transform;
use rand::{Rng, rng};

use noise::{BasicMulti, Perlin, utils::*};

use crate::common::AppState;
use crate::plugins::ui::ResetMapEvent;

#[derive(Resource, Deref)]
pub struct Root(Entity);

fn generate_noise_map() -> NoiseMap {
    let mut rng = rng();
    let seed: u32 = rng.random();

    let basicmulti = BasicMulti::<Perlin>::new(seed);
    let basicmulti = PlaneMapBuilder::new(&basicmulti).build();

    return basicmulti;
}

fn get_color(val: f64) -> Color {
    let color_result = match val.abs() {
        v if v < 0.1 => Color::from(Srgba::hex("#0a7e0a").unwrap()),
        v if v < 0.2 => Color::from(Srgba::hex("#0da50d").unwrap()),
        v if v < 0.3 => Color::from(Srgba::hex("#10cb10").unwrap()),
        v if v < 0.4 => Color::from(Srgba::hex("#18ed18").unwrap()),
        v if v < 0.5 => Color::from(Srgba::hex("#3ff03f").unwrap()),
        v if v < 0.6 => Color::from(Srgba::hex("#65f365").unwrap()),
        v if v < 0.7 => Color::from(Srgba::hex("#8cf68c").unwrap()),
        v if v < 0.8 => Color::from(Srgba::hex("#b2f9b2").unwrap()),
        v if v < 0.9 => Color::from(Srgba::hex("#d9fcd9").unwrap()),
        v if v <= 1.0 => Color::from(Srgba::hex("#ffffff").unwrap()),
        _ => panic!("unexpected value"),
    };
    return color_result;
}

pub fn generate_world(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
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

pub fn cleanup(mut commands: Commands, root: Res<Root>) {
    commands.entity(**root).despawn();
}

pub fn reset(
    mut events: MessageReader<ResetMapEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in events.read() {
        next_state.set(AppState::Build);
    }
}
