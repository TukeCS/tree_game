use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 0.0; // This is the player sprite size. (undetermined)

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, plant_tree_logic)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Tree;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/player.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(0.1)),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size - window.width() / 2.0;
        let x_max = window.width() / 2.0 - half_player_size;
        let y_min = half_player_size - window.height() / 2.0;
        let y_max = window.height() / 2.0 - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn plant_tree_logic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>, // Get player's position
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Get the player's position
        if let Some(player_transform) = query.iter().next() {
            commands.spawn((
                Tree,
                Sprite {
                    image: asset_server.load("sprites/tree.png"),
                    ..Default::default()
                },
                Transform {
                    translation: player_transform.translation,
                    scale: Vec3::splat(0.3),
                    ..Default::default()
                },
            ));
        }
    }
}
