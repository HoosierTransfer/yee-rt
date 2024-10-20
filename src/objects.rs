use nalgebra::Vector3;

use crate::transform::Transform;

pub struct Material {
    pub color: Vector3<f32>,
    pub roughness: f32,
    pub isMetal: bool,
    pub isDielectric: bool,
    pub ior: f32
}

pub trait Object {
    fn get_gpu_data(&self) -> Vec<u32>;
    fn get_gpu_data_custom_transform(&self, transform: &Transform) -> Vec<u32>;
    fn get_transform(&self) -> &Transform;
}

pub struct Sphere {
    pub material: Material,
    pub transform: Transform,
}

pub struct Box {
    pub material: Material,
    pub transform: Transform,
}

impl Sphere {
    pub fn new(transform: Transform, material: Material) -> Self {
        Sphere {
            material,
            transform
        }
    }
}

impl Object for Sphere {
    fn get_gpu_data(&self) -> Vec<u32> {
        let mut data = Vec::new();
        data.push(1);
        let transform = self.transform.get_model_matrix();
        data.push(transform[(0, 0)].to_bits());
        data.push(transform[(0, 1)].to_bits());
        data.push(transform[(0, 2)].to_bits());
        data.push(transform[(1, 0)].to_bits());
        data.push(transform[(1, 1)].to_bits());
        data.push(transform[(1, 2)].to_bits());
        data.push(transform[(2, 0)].to_bits());
        data.push(transform[(2, 1)].to_bits());
        data.push(transform[(2, 2)].to_bits());
        data.push(transform[(3, 0)].to_bits());
        data.push(transform[(3, 1)].to_bits());
        data.push(transform[(3, 2)].to_bits());
        data.push(self.material.color.x.to_bits());
        data.push(self.material.color.y.to_bits());
        data.push(self.material.color.z.to_bits());
        data.push(self.material.roughness.to_bits());
        data.push(self.material.isMetal as u32);
        data.push(self.material.isDielectric as u32);
        data.push(self.material.ior.to_bits());
        data
    }

    fn get_gpu_data_custom_transform(&self, transform: &Transform) -> Vec<u32> {
        let mut data = Vec::new();
        data.push(1);
        let transform = transform.get_model_matrix();
        data.push(transform[(0, 0)].to_bits());
        data.push(transform[(0, 1)].to_bits());
        data.push(transform[(0, 2)].to_bits());
        data.push(transform[(1, 0)].to_bits());
        data.push(transform[(1, 1)].to_bits());
        data.push(transform[(1, 2)].to_bits());
        data.push(transform[(2, 0)].to_bits());
        data.push(transform[(2, 1)].to_bits());
        data.push(transform[(2, 2)].to_bits());
        data.push(transform[(3, 0)].to_bits());
        data.push(transform[(3, 1)].to_bits());
        data.push(transform[(3, 2)].to_bits());
        data.push(self.material.color.x.to_bits());
        data.push(self.material.color.y.to_bits());
        data.push(self.material.color.z.to_bits());
        data.push(self.material.roughness.to_bits());
        data.push(self.material.isMetal as u32);
        data.push(self.material.isDielectric as u32);
        data.push(self.material.ior.to_bits());
        data
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Box {
    pub fn new(transform: Transform, material: Material) -> Self {
        Box {
            material,
            transform
        }
    }
}

impl Object for Box {
    fn get_gpu_data(&self) -> Vec<u32> {
        let mut data = Vec::new();
        data.push(2);
        let transform = self.transform.get_model_matrix();
        data.push(transform[(0, 0)].to_bits());
        data.push(transform[(0, 1)].to_bits());
        data.push(transform[(0, 2)].to_bits());
        data.push(transform[(1, 0)].to_bits());
        data.push(transform[(1, 1)].to_bits());
        data.push(transform[(1, 2)].to_bits());
        data.push(transform[(2, 0)].to_bits());
        data.push(transform[(2, 1)].to_bits());
        data.push(transform[(2, 2)].to_bits());
        data.push(transform[(3, 0)].to_bits());
        data.push(transform[(3, 1)].to_bits());
        data.push(transform[(3, 2)].to_bits());
        data.push(self.material.color.x.to_bits());
        data.push(self.material.color.y.to_bits());
        data.push(self.material.color.z.to_bits());
        data.push(self.material.roughness.to_bits());
        data.push(self.material.isMetal as u32);
        data.push(self.material.isDielectric as u32);
        data.push(self.material.ior.to_bits());
        data
    }

    fn get_gpu_data_custom_transform(&self, transform: &Transform) -> Vec<u32> {
        let mut data = Vec::new();
        data.push(2);
        let transform = transform.get_model_matrix();
        data.push(transform[(0, 0)].to_bits());
        data.push(transform[(0, 1)].to_bits());
        data.push(transform[(0, 2)].to_bits());
        data.push(transform[(1, 0)].to_bits());
        data.push(transform[(1, 1)].to_bits());
        data.push(transform[(1, 2)].to_bits());
        data.push(transform[(2, 0)].to_bits());
        data.push(transform[(2, 1)].to_bits());
        data.push(transform[(2, 2)].to_bits());
        data.push(transform[(3, 0)].to_bits());
        data.push(transform[(3, 1)].to_bits());
        data.push(transform[(3, 2)].to_bits());
        data.push(self.material.color.x.to_bits());
        data.push(self.material.color.y.to_bits());
        data.push(self.material.color.z.to_bits());
        data.push(self.material.roughness.to_bits());
        data.push(self.material.isMetal as u32);
        data.push(self.material.isDielectric as u32);
        data.push(self.material.ior.to_bits());
        data
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}