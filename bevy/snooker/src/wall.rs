use crate::config::CONFIG;
use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

fn create_horizontal_wall_sprite() -> Sprite {
    Sprite {
        color: CONFIG.wall_color,
        custom_size: Some(Vec2::new(CONFIG.table_size.x as f32, CONFIG.wall_width)),

        ..default()
    }
}

fn create_vertical_wall_sprite() -> Sprite {
    Sprite {
        color: CONFIG.wall_color,
        custom_size: Some(Vec2::new(CONFIG.wall_width, CONFIG.table_size.y as f32)),

        ..default()
    }
}

fn create_wall_transform(x: f32, y: f32) -> Transform {
    Transform::from_translation(Vec3::new(x, y, 10.0))
}

pub fn setup_walls(
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            sprite: create_horizontal_wall_sprite(),
            transform: create_wall_transform(
                0.0,
                (CONFIG.wall_width / 2.0) - (CONFIG.table_size.y as f32) / 2.0,
            ),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: create_horizontal_wall_sprite(),
            transform: create_wall_transform(
                0.0,
                -(CONFIG.wall_width / 2.0) + (CONFIG.table_size.y as f32) / 2.0,
            ),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: create_vertical_wall_sprite(),
            transform: create_wall_transform(
                (CONFIG.wall_width / 2.0) - (CONFIG.table_size.x as f32) / 2.0,
                0.0
            ),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: create_vertical_wall_sprite(),
            transform: create_wall_transform(
                -(CONFIG.wall_width / 2.0) + (CONFIG.table_size.x as f32) / 2.0,
                0.0
            ),
            ..default()
        },
        Wall,
    ));

}
