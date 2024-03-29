use bevy::prelude::*;

use rand::random;

use crate::config::CONFIG;

pub fn setup_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 1..CONFIG.number_of_stars {
        let star_location = (Vec3::ONE / 2.0
            - Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()))
            * CONFIG.planet_radius
            * 1.9;

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: random::<f32>() * 2.0,
                        subdivisions: 1,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: Color::WHITE,
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(star_location),
                ..default()
            });
    }
}
