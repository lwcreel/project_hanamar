// Bevy Imports
use bevy::prelude::Color;
use bevy::prelude::*;

use bevy::{
    color::palettes::tailwind::RED_400,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

// Other Imports
use noise::{BasicMulti, Perlin, utils::*};
use rand::{Rng, rng};

// Custom Imports
use crate::plugins::states::GameState;
use crate::plugins::ui::ExitEvent;

#[derive(Component, Deref, DerefMut)]
pub struct UpdateTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct Tiles(Vec<Option<TileData>>);

#[derive(Component)]
pub struct MovePlayer;

#[derive(Resource, Deref)]
pub struct Root(Entity);

fn generate_noise_map() -> NoiseMap {
    PlaneMapBuilder::new(&BasicMulti::<Perlin>::new(rng().random())).build()
}

pub fn cleanup(mut commands: Commands, root: Res<Root>) {
    commands.remove_resource::<Tiles>();
    commands.entity(**root).despawn();
}

pub fn reset(mut events: MessageReader<ExitEvent>, mut next_state: ResMut<NextState<GameState>>) {
    for _ in events.read() {
        next_state.set(GameState::Menu);
    }
}

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let map = generate_noise_map();

    let chunk_size = UVec2::splat(64);
    let tile_display_size = UVec2::splat(8);

    let mut tile_vec: Vec<u16> = vec![];

    for col_x in 0..chunk_size.x {
        for col_y in 0..chunk_size.y {
            tile_vec.push(((map.get_value(col_x as usize, col_y as usize) + 0.5) * 5.0) as u16);
        }
    }

    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|i| tile_vec.get(i as usize))
        .map(|val| {
            let val = *val.unwrap();
            println!("{}", val);
            if val == 0 {
                None
            } else {
                Some(TileData::from_tileset_index(val - 1))
            }
        })
        .collect();

    // Clone Tile Data as a Resource
    commands.insert_resource(Tiles(tile_data.clone()));
    commands.spawn(Camera2d);

    let root = commands
        .spawn((Transform::default(), Visibility::default()))
        .with_child((
            TilemapChunk {
                chunk_size,
                tile_display_size,
                tileset: assets.load("textures/array_texture.png"),
                ..default()
            },
            TilemapChunkTileData(tile_data),
            UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .id();

    commands.insert_resource(Root(root));
}

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
    tiles: ResMut<Tiles>,
) {
    for (mut tile_data, mut timer) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            for i in 0..tile_data.len() {
                tile_data[i] = Some(tiles[i].unwrap_or_default());
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
