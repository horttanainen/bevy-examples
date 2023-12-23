use crate::config::CONFIG;



pub const BALL_BUFFER_SIZE: u64 = ((std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<i32>())
                                    * CONFIG.number_of_balls as usize)
                                    as u64;

pub const POCKET_BUFFER_SIZE: u64 = ((std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<i32>())
                                    * 6 as usize)
                                    as u64;

pub const CUE_BALL_BUFFER_SIZE: u64 = (std::mem::size_of::<f32>() * 2) as u64;

pub const TIME_BUFFER_SIZE: u64 = std::mem::size_of::<f32>() as u64;
