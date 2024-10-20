use nalgebra::{Vector3, Matrix4};
use std::ops::Mul;

pub struct Transform {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Vector3<f32>
}

impl Transform {
    pub fn new(position: Vector3<f32>, scale: Vector3<f32>, rotation: Vector3<f32>) -> Self {
        Transform {
            position,
            scale,
            rotation
        }
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let mut translation = Matrix4::identity();
        translation[(3, 0)] = self.position.x;
        translation[(3, 1)] = self.position.y;
        translation[(3, 2)] = self.position.z;
        let inverse_scale = Vector3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
        let scale = Matrix4::new_nonuniform_scaling(&inverse_scale);
        let rotation = Matrix4::from_euler_angles(self.rotation.x.to_radians(), self.rotation.y.to_radians(), self.rotation.z.to_radians());
        translation * rotation * scale
    }

    pub fn clone(&self) -> Self {
        Transform {
            position: self.position,
            scale: self.scale,
            rotation: self.rotation
        }
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let scale = Vector3::new(self.scale.x * rhs.scale.x, self.scale.y * rhs.scale.y, self.scale.z * rhs.scale.z);
        let rotation = self.rotation + rhs.rotation;
        Transform {
            position,
            scale,
            rotation
        }
    }
}