use std::time::Duration;

use glm::{Matrix4, Vec3, Vector3};
use once_cell::sync::Lazy;

/// global up vector
static UP: Lazy<Vector3<f32>> = Lazy::new(|| Vector3::new(0.0, 1.0, 0.0));
const NEAR: f32 = 0.1;
const FAR: f32 = 100.0;
const ASPECT: f32 = 8.0 / 5.0;
const FOV: f32 = 45.0;

#[derive(Debug)]
pub struct Camera {
    pos: Vector3<f32>,
    pitch: f32, // [-89, 89]
    yaw: f32,
    front: Vector3<f32>,
    move_speed: f32,
    mouse_speed: f32,
}

pub struct CameraBuilder {
    inner: Camera,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            inner: Camera {
                pos: Vector3::new(0.0, 0.0, 3.0),
                pitch: 0.0,
                yaw: 90.0,
                front: Vector3::new(0.0, 0.0, -1.0),
                move_speed: 2.5,
                mouse_speed: 40.0,
            },
        }
    }
}

impl CameraBuilder {
    pub fn pos(mut self, pos: Vector3<f32>) -> Self {
        self.inner.pos = pos;
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
    /// pitch in degrees, [-89, 89]
    pub fn pitch(mut self, pitch: f32) -> Self {
        self.inner.pitch = pitch;
        self
    }

    /// yaw in degrees
    pub fn yaw(mut self, yaw: f32) -> Self {
        self.inner.yaw = yaw;
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
                self.pos =
                    self.pos - glm::normalize(glm::cross(self.front, UP.clone())) * move_speed;
            }
            CameraMovement::Right => {
                self.pos =
                    self.pos + glm::normalize(glm::cross(self.front, UP.clone())) * move_speed;
            }
            CameraMovement::Up => {
                self.pos = self.pos + UP.clone() * move_speed;
            }
            CameraMovement::Down => {
                self.pos = self.pos - UP.clone() * move_speed;
            }
            CameraMovement::Rotate(x, y) => {
                let x = x * mouse_speed;
                let y = y * mouse_speed;
                self.pitch -= y;
                if self.pitch > 89.0 {
                    self.pitch = 89.0;
                } else if self.pitch < -89.0 {
                    self.pitch = -89.0;
                }

                self.yaw -= x;
                if self.yaw > 360.0 {
                    self.yaw -= 360.0;
                } else if self.yaw < 0.0 {
                    self.yaw += 360.0;
                }

                self.front = self.calc_front();
            }
        }
    }

    pub fn get_mat(&self) -> Matrix4<f32> {
        let view_mat = glm::ext::look_at(self.pos, self.pos + self.front, UP.clone());
        let proj_mat = glm::ext::perspective(FOV, ASPECT, NEAR, FAR);
        proj_mat * view_mat
    }

    pub fn get_camera_pos(&self) -> Vec3 {
        self.pos
    }

    /// return normalized front vector
    fn calc_front(&self) -> Vector3<f32> {
        // (cos(yaw), tan(pitch), -sin(yaw))
        let yaw = glm::radians(self.yaw);
        let pitch = glm::radians(self.pitch);
        let front = Vector3::new(yaw.cos(), pitch.tan(), -yaw.sin());
        glm::normalize(front)
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
