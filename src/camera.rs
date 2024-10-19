use nalgebra::{Vector3, Matrix4, Rotation3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub world_up: Vector3<f32>,
    pub front: Vector3<f32>,
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
    pub euler_angle: Vector3<f32>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, up: Vector3<f32>, euler_angle: Vector3<f32>) -> Self {
        let mut camera = Camera {
            position,
            world_up: up,
            front: Vector3::zeros(),
            right: Vector3::zeros(),
            up: Vector3::zeros(),
            euler_angle,
        };
        camera.update_camera_vectors();
        camera
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_euler_angle(&mut self, euler_angle: Vector3<f32>) {
        self.euler_angle = euler_angle;
        self.update_camera_vectors();
    }

    pub fn move_camera(&mut self, offset: Vector3<f32>) {
        self.position += offset;
    }

    pub fn rotate(&mut self, angle: f32, axis: Vector3<f32>) {
        let rotation = Matrix4::from(Rotation3::new(axis * angle.to_radians()));
        let new_front = rotation.transform_vector(&self.front);
        self.euler_angle.y = new_front.y.asin().to_degrees();
        self.euler_angle.x = new_front.z.atan2(new_front.x).to_degrees();
        self.update_camera_vectors();
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position.into(), &(self.position + self.front).into(), &self.up)
    }

    pub fn process_keyboard(&mut self, direction: &str, delta_time: f32) {
        let velocity = 2.5 * delta_time;
        match direction {
            "FORWARD" => self.position += self.front * velocity,
            "BACKWARD" => self.position -= self.front * velocity,
            "LEFT" => self.position -= self.right * velocity,
            "RIGHT" => self.position += self.right * velocity,
            "UP" => self.position += self.up * velocity,
            "DOWN" => self.position -= self.up * velocity,
            _ => (),
        }
        
    }

    pub fn process_mouse_movement(&mut self, xoffset: f32, yoffset: f32, constrain_pitch: bool) {
        let sensitivity = 0.1;
        let xoffset = xoffset * sensitivity;
        let yoffset = yoffset * sensitivity;

        self.euler_angle.x += xoffset;
        self.euler_angle.y += yoffset;

        if constrain_pitch {
            if self.euler_angle.y > 89.0 {
                self.euler_angle.y = 89.0;
            }
            if self.euler_angle.y < -89.0 {
                self.euler_angle.y = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    fn update_camera_vectors(&mut self) {
        let front = Vector3::new(
            self.euler_angle.x.to_radians().cos() * self.euler_angle.y.to_radians().cos(),
            self.euler_angle.y.to_radians().sin(),
            self.euler_angle.x.to_radians().sin() * self.euler_angle.y.to_radians().cos(),
        );

        self.front = front.normalize();
        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}
