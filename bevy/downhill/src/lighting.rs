use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::config::CONFIG;

pub fn setup_lighting(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 400.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            maximum_distance: CONFIG.planet_radius * 2.,
            ..default()
        }
        .into(),
        ..default()
    });
}
