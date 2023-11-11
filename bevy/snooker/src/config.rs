use bevy::prelude::Color;

pub struct Config {
    pub size: (u32, u32),
    pub workgroup_size: u32,
    pub ball_radius: f32,
    pub number_of_balls: usize,
    pub wall_width: f32,
    pub wall_color: Color,
    pub pocket_radius: f32
}

pub const CONFIG: Config = Config {
    size: (1280, 1280 / 2),
    workgroup_size: 8,
    ball_radius: 10.,
    number_of_balls: 10,
    wall_width: 20.0,
    wall_color: Color::TEAL,
    pocket_radius: 12.
};
