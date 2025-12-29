use bevy::prelude::Color;
use bevy::prelude::*;

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
        v if v < 0.1 => Color::from(bevy::color::Hsla::hsl(value, 0.75, 0.0)),
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

pub fn generate_world(mut commands: Commands, color: ResMut<Hue>) {
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
}

pub fn cleanup(mut commands: Commands, root: Res<Root>) {
    commands.entity(**root).despawn();
}

pub fn reset(
    mut events: MessageReader<ResetMapEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        next_state.set(GameState::Menu);
    }
}

use bevy::{
    color::palettes::tailwind::RED_400,
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

//        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//        app.add_systems(Startup, (setup, spawn_fake_player).chain())
//        app.add_systems(
//            Update,
//            (update_tileset_image, update_tilemap, move_player, log_tile),
//        )

#[derive(Component, Deref, DerefMut)]
pub struct UpdateTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct SeededRng(ChaCha8Rng);

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // We're seeding the PRNG here to make this example deterministic for testing purposes.
    // This isn't strictly required in practical use unless you need your app to be deterministic.
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let chunk_size = UVec2::splat(64);
    let tile_display_size = UVec2::splat(8);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| rng.random_range(0..5))
        .map(|i| {
            if i == 0 {
                None
            } else {
                Some(TileData::from_tileset_index(i - 1))
            }
        })
        .collect();

    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load("textures/array_texture.png"),
            ..default()
        },
        TilemapChunkTileData(tile_data),
        UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    commands.spawn(Camera2d);

    commands.insert_resource(SeededRng(rng));
}

#[derive(Component)]
pub struct MovePlayer;

pub fn spawn_fake_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    chunk: Single<&TilemapChunk>,
) {
    let mut transform = chunk.calculate_tile_transform(UVec2::new(0, 0));
    transform.translation.z = 1.;

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(8., 8.))),
        MeshMaterial2d(materials.add(Color::from(RED_400))),
        transform,
        MovePlayer,
    ));

    let mut transform = chunk.calculate_tile_transform(UVec2::new(5, 6));
    transform.translation.z = 1.;

    // second "player" to visually test a non-zero position
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(8., 8.))),
        MeshMaterial2d(materials.add(Color::from(RED_400))),
        transform,
    ));
}

pub fn move_player(
    mut player: Single<&mut Transform, With<MovePlayer>>,
    time: Res<Time>,
    chunk: Single<&TilemapChunk>,
) {
    let t = (ops::sin(time.elapsed_secs()) + 1.) / 2.;

    let origin = chunk
        .calculate_tile_transform(UVec2::new(0, 0))
        .translation
        .x;
    let destination = chunk
        .calculate_tile_transform(UVec2::new(63, 0))
        .translation
        .x;

    player.translation.x = origin.lerp(destination, t);
}

pub fn update_tileset_image(
    chunk_query: Single<&TilemapChunk>,
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    let chunk = *chunk_query;
    for event in events.read() {
        if event.is_loaded_with_dependencies(chunk.tileset.id()) {
            let image = images.get_mut(&chunk.tileset).unwrap();
            image.reinterpret_stacked_2d_as_array(4);
        }
    }
}

pub fn update_tilemap(
    time: Res<Time>,
    mut query: Query<(&mut TilemapChunkTileData, &mut UpdateTimer)>,
    mut rng: ResMut<SeededRng>,
) {
    for (mut tile_data, mut timer) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            for _ in 0..50 {
                let index = rng.random_range(0..tile_data.len());
                tile_data[index] = Some(TileData::from_tileset_index(rng.random_range(0..5)));
            }
        }
    }
}

// find the data for an arbitrary tile in the chunk and log its data
pub fn log_tile(tilemap: Single<(&TilemapChunk, &TilemapChunkTileData)>, mut local: Local<u16>) {
    let (chunk, data) = tilemap.into_inner();
    let Some(tile_data) = data.tile_data_from_tile_pos(chunk.chunk_size, UVec2::new(3, 4)) else {
        return;
    };
    // log when the tile changes
    if tile_data.tileset_index != *local {
        info!(?tile_data, "tile_data changed");
        *local = tile_data.tileset_index;
    }
}
