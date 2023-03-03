use glium::{
    Display, DrawParameters, Frame, implement_vertex, IndexBuffer, Program, Surface, uniform,
    VertexBuffer,
};
use glm::{Mat4, Vec3};

use crate::Drawable;

pub struct Light {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: IndexBuffer<u8>,
    light_color: (f32, f32, f32),
    light_pos: Vec3,
}

impl Light {
    fn new(display: &Display) -> Self {
        let program = Program::from_source(
            display,
            include_str!("shader.vert"),
            include_str!("shader.frag"),
            None,
        )
        .unwrap();

        let shape = vec![
            Vertex::new(0.5, 0.5, 0.5),
            Vertex::new(0.5, -0.5, 0.5),
            Vertex::new(-0.5, -0.5, 0.5),
            Vertex::new(-0.5, 0.5, 0.5),
            Vertex::new(0.5, 0.5, -0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(-0.5, 0.5, -0.5),
        ];

        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();

        let indices = glium::index::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &[
                0u8, 3, 1, 3, 2, 1, 0, 1, 5, 5, 4, 0, 4, 5, 6, 6, 7, 4, 7, 6, 2, 2, 3, 7, 4, 7, 3,
                3, 0, 4, 2, 6, 5, 5, 1, 2,
            ],
        )
        .unwrap();

        Self {
            program,
            vertex_buffer,
            indices,
            light_color: (1.0, 1.0, 1.0),
            light_pos: Vec3::new(2.0, 2.0, 2.0),
        }
    }

    pub fn set_light_color(&mut self, color: (f32, f32, f32)) {
        self.light_color = color;
    }

    pub fn set_light_pos(&mut self, pos: Vec3) {
        self.light_pos = pos;
    }
}

impl Drawable for Light {
    fn init(display: &Display) -> Self {
        Self::new(display)
    }

    fn draw_with_frame(&self, frame: &mut Frame, camera_mat: Mat4, dramparams: &DrawParameters) {
        let uniforms = uniform! {
            camera: [
                *camera_mat.as_array()[0].as_array(),
                *camera_mat.as_array()[1].as_array(),
                *camera_mat.as_array()[2].as_array(),
                *camera_mat.as_array()[3].as_array(),
            ],
            shift: [self.light_pos.x, self.light_pos.y, self.light_pos.z],
            light_color: [self.light_color.0, self.light_color.1, self.light_color.2],
        };

        frame
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms,
                dramparams,
            )
            .unwrap();
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 3],
}
implement_vertex!(Vertex, pos location(0));

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { pos: [x, y, z] }
    }
}
