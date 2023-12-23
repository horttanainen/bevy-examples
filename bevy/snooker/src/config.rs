use bevy::prelude::*;

pub struct Config {
    pub table_size: IVec2,
    pub workgroup_size: u32,
    pub ball_radius: f32,
    pub cue_ball_starting_position: Vec3,
    pub number_of_balls: i32,
    pub wall_width: f32,
    pub wall_color: Color,
    pub pocket_radius: f32
}

pub const CONFIG: Config = Config {
    table_size: IVec2::new(1280, 1280 / 2),
    workgroup_size: 8,
    ball_radius: 10.,
    cue_ball_starting_position: Vec3::new(0.0, 0.0, 20.0),
    number_of_balls: 10,
    wall_width: 20.0,
    wall_color: Color::TEAL,
    pocket_radius: 12.
};
