pub struct Config {
    pub size: (u32, u32),
    pub workgroup_size: u32,
}

pub const CONFIG: Config = Config {
    size: (1280, 1280),
    workgroup_size: 8,
};
