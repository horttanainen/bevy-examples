use bevy::prelude::*;

use crate::{config::CONFIG, player::Player, directions::Direction};

#[derive(Component)]
pub struct PlayerSpot;


pub fn setup_lighting(
    mut commands: Commands,
) {
    commands.spawn(SpotLightBundle {
        spot_light: SpotLight {
            intensity: 8000.,
            range: CONFIG.planet_radius * 1.5,
            inner_angle: std::f32::consts::FRAC_PI_8,
            outer_angle: std::f32::consts::FRAC_PI_6,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::ZERO,
            ..default()
        }
        .into(),
        ..default()
    })
    .insert(PlayerSpot);
}

pub fn move_spotlight(
    player_q: Query<&Transform, With<Player>>,
    mut spot_q: Query<&mut Transform, (With<PlayerSpot>, Without<Player>)>,
    direction: Res<Direction>,
) {

    let mut spot_t = spot_q.single_mut();
    let player_t = player_q.single();

    spot_t.translation = player_t.translation + direction.player_up * 50.0;
    spot_t.look_at(player_t.translation, Vec3::Y);
}




