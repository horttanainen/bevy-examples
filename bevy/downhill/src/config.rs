use bevy::prelude::Vec3;


pub struct Config {
    pub planet_radius: f32,
    pub planet_center: Vec3,
    pub player_mass: f32,
    pub gravity: f32
}

pub const CONFIG: Config = Config {
    planet_radius: 100.,
    planet_center: Vec3::ZERO,
    player_mass: 100.0,
    gravity: 9.81
};
