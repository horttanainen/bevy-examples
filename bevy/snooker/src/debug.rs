use bevy::prelude::*;

use crate::camera::MainCamera;

pub fn draw_viewport_rect(mut gizmos: Gizmos, q: Query<&Camera, With<MainCamera>>) {
    let camera = q.single();

    // the top-left and bottom-right coordinates
    let view_port_rect = camera.logical_viewport_rect().unwrap();

    gizmos.rect_2d(
        view_port_rect.min,
        0.0,
        view_port_rect.size(),
        Color::RED,
    );
}
