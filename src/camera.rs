use std::time::Duration;
use glm::{Matrix4, Vector3};

#[derive(Debug)]
pub struct Camera {
    pos: Vector3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    move_speed: f32,
    mouse_speed: f32,

    near: f32,
    far: f32,
    fov: f32,
    aspect: f32,
}

pub struct CameraBuilder {
    inner: Camera,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            inner: Camera {
                pos: Vector3::new(0.0, 0.0, 5.0),
                front: Vector3::new(0.0, 0.0, 0.0),
                up: Vector3::new(0.0, 1.0, 0.0),
                move_speed: 2.5,
                mouse_speed: 0.01,
                near: 0.1,
                far: 100.0,
                fov: 45.0,
                aspect: 1.0,
            },
        }
    }
}

impl CameraBuilder {
    pub fn pos(mut self, pos: Vector3<f32>) -> Self {
        self.inner.pos = pos;
        self
    }

    pub fn front(mut self, front: Vector3<f32>) -> Self {
        self.inner.front = front;
        self
    }

    pub fn up(mut self, up: Vector3<f32>) -> Self {
        self.inner.up = up;
        self
    }

    pub fn move_speed(mut self, move_speed: f32) -> Self {
        self.inner.move_speed = move_speed;
        self
    }

    pub fn mouse_speed(mut self, mouse_speed: f32) -> Self {
        self.inner.mouse_speed = mouse_speed;
        self
    }

    pub fn build(self) -> Camera {
        self.inner
    }
}

impl Camera {
    pub fn new() -> Self {
        CameraBuilder::default().build()
    }

    pub fn update(&mut self, delta_time: Duration, movement: CameraMovement) {
        let delta_time = delta_time.as_secs_f32();
        let move_speed = self.move_speed * delta_time;
        let mouse_speed = self.mouse_speed * delta_time;
        match movement {
            CameraMovement::Forward => {
                self.pos = self.pos + self.front * move_speed;
            }
            CameraMovement::Backward => {
                self.pos = self.pos - self.front * move_speed;
            }
            CameraMovement::Left => {
                self.pos = self.pos - glm::normalize(glm::cross(self.front, self.up)) * move_speed;
            }
            CameraMovement::Right => {
                self.pos = self.pos + glm::normalize(glm::cross(self.front, self.up)) * move_speed;
            }
            CameraMovement::Up => {
                self.pos = self.pos + self.up * move_speed;
            }
            CameraMovement::Down => {
                self.pos = self.pos - self.up * move_speed;
            }
            CameraMovement::Rotate(x, y) => {
                let x = x * mouse_speed;
                let y = y * mouse_speed;
                let mut front = self.front;
                front.x = (front.x * x.cos() - front.z * x.sin()).cos() * front.y.cos();
                front.y = front.y.sin() * y.sin() + front.y.cos() * y.cos();
                front.z = (front.z * x.cos() + front.x * x.sin()).sin() * front.y.cos();
                self.front = front;
            }
        }
        println!("{self:#?}");
    }

    pub fn get_mat(&self) -> Matrix4<f32> {
        let mut mat = glm::ext::look_at(self.pos, self.pos + self.front, self.up);
        mat * glm::ext::perspective(glm::radians(self.fov), self.aspect, self.near, self.far)
    }
}

#[derive(Clone, Copy)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    Rotate(f32, f32),
}
