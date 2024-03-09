use bevy::prelude::*;

use crate::{
    config::CONFIG, directions::Direction, player::Player, velocity::NonZeroSurfaceVelocity,
};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50., 400.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);
}

pub fn move_camera(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    surface_velocity: Res<NonZeroSurfaceVelocity>,
    direction: Res<Direction>,
) {
    let mut camera_transform = camera_q.single_mut();
    let player_transform = player_q.single();

    let distance_to_player =
        Vec3::distance(camera_transform.translation, player_transform.translation);

    let player_surface_velocity = &surface_velocity.0;

    let max_visible_speed = 30.;
    let min_horizontal_camera_distance = 5.;
    let max_horizontal_camera_distance = 10.;

    let max_visible_elevation = 30.;
    let min_vertical_camera_distance = 2.;
    let max_vertical_camera_distance = 5.;

    let from_surface_to_player = player_transform.translation - direction.planet_center;
    let elevation = from_surface_to_player.length() - CONFIG.planet_radius;

    let above_player = direction.player_up
        * f32::max(
            (elevation.min(max_visible_elevation) / max_visible_elevation)
                * max_vertical_camera_distance,
            min_vertical_camera_distance,
        );

    let behind_player = player_surface_velocity.normalize()
        * f32::max(
            (player_surface_velocity
                .clamp_length_max(max_visible_speed)
                .length()
                / max_visible_speed)
                * max_horizontal_camera_distance,
            min_horizontal_camera_distance,
        );

    if player_surface_velocity.length() > 0.5 {
        camera_transform.translation = player_transform.translation - behind_player + above_player;
    } else if distance_to_player > 20. {
        camera_transform.translation = player_transform.translation + above_player;
    }

    camera_transform.look_at(player_transform.translation, direction.player_up);
}
