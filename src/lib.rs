use glium::{Display, DrawParameters, Frame};
use glm::Mat4;

pub mod camera;
pub mod camera_events;
pub mod refresh_rate;

pub mod hair_cube;
pub mod light_source;

pub trait Drawable {
    fn init(display: &Display) -> Self;
    fn draw_with_frame(&self, frame: &mut Frame, camera_mat: Mat4, dramparams: &DrawParameters);
}
