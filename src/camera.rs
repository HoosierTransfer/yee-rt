use nalgebra::{Vector3, Matrix4, Unit};
use std::f32::consts::PI;

pub struct Camera {
    position: Vector3<f32>,
    world_up: Vector3<f32>,
    front: Vector3<f32>,
    right: Vector3<f32>,
    up: Vector3<f32>,
    euler_angle: Vector3<f32>,
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
        let rotation = Matrix4::rotation(axis, angle.to_radians());
        let new_front = rotation.transform_vector(&self.front);
        self.euler_angle.y = new_front.y.asin().to_degrees();
        self.euler_angle.x = new_front.z.atan2(new_front.x).to_degrees();
        self.update_camera_vectors();
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn process_keyboard(&mut self, window: &glfw::Window, delta_time: f32) {
        let camera_speed = 2.5 * delta_time;

        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            self.position += camera_speed * self.front;
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
