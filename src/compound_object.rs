use crate::objects::Object;
use crate::transform::Transform;

pub struct CompoundObject {
    objects: Vec<Box<dyn Object>>,
    pub transform: Transform,
}

impl Object for CompoundObject {
    fn get_gpu_data(&self) -> Vec<u32> {
        let mut data = Vec::new();
        for object in &self.objects {
            let transform = self.transform.clone() * object.get_transform().clone();
            let mut object_data = object.get_gpu_data_custom_transform(&transform);
            data.append(&mut object_data);
        }
        data
    }

    fn get_gpu_data_custom_transform(&self, transform: &Transform) -> Vec<u32> {
        let mut data = Vec::new();
        for object in &self.objects {
            let transform1 = transform.clone() * object.get_transform().clone();
            let mut object_data = object.get_gpu_data_custom_transform(&transform1);
            data.append(&mut object_data);
        }
        data
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl CompoundObject {
    pub fn new(transform: Transform) -> Self {
        CompoundObject {
            objects: Vec::new(),
            transform,
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
}