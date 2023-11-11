use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

use crate::{camera::MainCamera, config::CONFIG};

pub fn handle_cursor(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rapier_context: Res<RapierContext>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        eprintln!("Mouse clicked!!!");

        if let Some(entity) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .and_then(|ray| {
                rapier_context.cast_ray(
                    ray.origin,
                    ray.direction,
                    1000.0,
                    true,
                    QueryFilter::default(),
                )
            })
        {
            eprintln!("Found entity!!!");
            let highlight = commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(CONFIG.ball_radius + 5.0).into())
                    .into(),
                    material: materials.add(ColorMaterial::from(Color::GOLD)),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
                    ..default()
            }).id();

            commands.entity(entity.0).add_child(highlight);
        }
    }
}
