use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::random;

pub const TABLE_SIZE: IVec2 = IVec2::new(100, 100);
pub const TILE_SIZE: Vec2 = Vec2::new(10., 10.);
pub const BALL_RADIUS: f32 = 10.;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct CueBall;

#[derive(Component)]
pub struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_gizmos, color_tiles, move_cue_ball))
        .run();
}

fn color_tiles(
    mut materials: ResMut<Assets<ColorMaterial>>,
    tiles: Query<(&mut Handle<ColorMaterial>, &Transform), With<Tile>>,
    que_ball: Query<&Transform, With<CueBall>>,
    balls: Query<&Transform, With<Ball>>,
) {
    let cue_ball_t = que_ball.single();
    let ball_positions = balls.iter().map(|t| t.translation).collect();
    for (cm, tile_t) in &tiles {
        let color_mat = materials.get_mut(&cm).unwrap();
        if !can_see_cue_ball(tile_t.translation, cue_ball_t.translation, &ball_positions) {
            color_mat.color = Color::rgb(random::<f32>(), random::<f32>(), random::<f32>());
        } else if color_mat.color != Color::GREEN {
            color_mat.color = Color::GREEN;
        }
    }
}

fn move_cue_ball(
    mut cue_ball: Query<(&mut Transform, With<CueBall>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut cue_ball {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}

fn can_see_cue_ball(tile_pos: Vec3, cue_ball_pos: Vec3, ball_positions: &Vec<Vec3>) -> bool {
    let tile_pos2 = Vec3::new(tile_pos.x, tile_pos.y, 0.);
    let cue_ball_pos2 = Vec3::new(cue_ball_pos.x, cue_ball_pos.y, 0.);
    for ball_pos in ball_positions {
        let ball_pos2 = Vec3::new(ball_pos.x, ball_pos.y, 0.);

        let ball_to_cue_ball = cue_ball_pos2 - ball_pos2;
        let tile_to_ball = ball_pos2 - tile_pos2;
        let tile_to_cue_ball = cue_ball_pos2 - tile_pos2;

        if Vec3::length_squared(tile_to_cue_ball) < Vec3::length_squared(tile_to_ball)
            || Vec3::length_squared(tile_to_cue_ball) < Vec3::length_squared(ball_to_cue_ball)
        {
            continue;
        }
        let towards_cue_ball_until_ball = Vec3::dot(tile_to_ball, tile_to_cue_ball)
            / Vec3::length_squared(tile_to_cue_ball)
            * tile_to_cue_ball;
        let distance = Vec3::length(tile_to_ball - towards_cue_ball_until_ball);
        if distance <= BALL_RADIUS / 2. {
            return false;
        }
    }
    true
}

fn draw_gizmos(mut gizmos: Gizmos, tiles: Query<&Transform, With<Tile>>) {
    for t in &tiles {
        gizmos.rect_2d(
            Vec2::new(t.translation.x, t.translation.y),
            0.,
            Vec2::new(TILE_SIZE.x, TILE_SIZE.y),
            Color::RED,
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for x in 0..TABLE_SIZE.x {
        for y in 0..TABLE_SIZE.y {
            let mut pos = IVec2::new(x - TABLE_SIZE.x / 2, y - TABLE_SIZE.y / 2);
            pos *= IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2::new(TILE_SIZE.x, TILE_SIZE.y)).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform::from_translation(Vec3::new(
                        pos.x as f32,
                        pos.y as f32,
                        0.,
                    )),
                    ..default()
                },
                Tile,
            ));
        }
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::ONE),
            ..default()
        },
        CueBall,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(Vec3::ONE * 50.),
            ..default()
        },
        Ball,
    ));
}
