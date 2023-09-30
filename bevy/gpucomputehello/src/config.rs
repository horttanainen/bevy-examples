pub struct Config {
    pub size: (u32, u32),
    pub workgroup_size: u32,
    pub ball_radius: f32,
    pub number_of_balls: usize
}

pub const CONFIG: Config = Config {
    size: (1280, 1280),
    workgroup_size: 8,
    ball_radius: 10.,
    number_of_balls: 10
};
