use bevy::prelude::*;

use crate::plugins::game_menu::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Setup), setup_camera);
        app.add_systems(
            Update,
            camera_movement_system.run_if(in_state(GameState::Game)),
        );
    }
}

/// 3D Orthographic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut projection)) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        let ortho = match &mut *projection {
            Projection::Orthographic(ortho) => ortho,
            _ => unimplemented!("Only OrthographicProjection is implemented in this example"),
        };

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let translation = &mut transform.translation;
        *translation += time.delta_secs() * 500.0 * direction;
    }
}
