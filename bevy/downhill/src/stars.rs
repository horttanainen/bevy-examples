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
            * 5000.0;
        let outside_planet = star_location.clamp_length_min(CONFIG.planet_radius + 1000.0);

        commands.spawn(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: random::<f32>() * 10.0,
                    subdivisions: 1,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: Color::WHITE,
                ..default()
            }),
            transform: Transform::from_translation(outside_planet),
            ..default()
        });
    }
}
