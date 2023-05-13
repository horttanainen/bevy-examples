use cgmath::{Zero, Rotation3, InnerSpace};

use crate::instance_raw::InstanceRaw;


pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub frame: u32,
    pub original_position: cgmath::Vector3<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }

    pub fn create_instances(number_of_instances_per_row: u32, instance_displacement: cgmath::Vector3<f32>) -> Vec<Instance> {
        let instances = (0..number_of_instances_per_row)
            .flat_map(|z| {
                (0..number_of_instances_per_row).map(move |x| {
                    let z_normalized: f32 = z as f32 / number_of_instances_per_row as f32;
                    let x_normalized: f32 = x as f32 / number_of_instances_per_row as f32;

                    let position = cgmath::Vector3 {
                        x: x as f32,
                        y: cgmath::Angle::sin(cgmath::Rad(
                            z_normalized * x_normalized * 10.0 * 2.0 * std::f32::consts::PI,
                        )),
                        z: z as f32,
                    } - instance_displacement;

                    let rotation = if position.is_zero() {
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not created correctly
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(
                            position.normalize(),
                            cgmath::Rad(x_normalized * 2.0 * std::f32::consts::PI),
                        )
                    };

                    Instance {
                        position,
                        rotation,
                        frame: 0,
                        original_position: position,
                    }
                })
            })
            .collect::<Vec<_>>();

            instances
    }
}

