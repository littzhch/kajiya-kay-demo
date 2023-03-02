use crate::Drawable;
use glium::texture::SrgbTexture2d;
use glium::uniforms::Sampler;
use glium::{
    implement_vertex, texture, uniform, Display, DrawParameters, Frame, IndexBuffer, Program,
    Surface, VertexBuffer,
};
use glm::Mat4;
use image::ImageFormat;
use std::io::Cursor;

pub struct HairCube {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: IndexBuffer<u8>,
    texture: SrgbTexture2d,
    light_color: (f32, f32, f32),
}

impl Drawable for HairCube {
    fn init(display: &Display) -> Self {
        let program = Program::from_source(
            display,
            include_str!("shader.vert"),
            include_str!("shader.frag"),
            None,
        )
        .unwrap();

        let shape = vec![
            Vertex::new((0.5, 0.5, 0.5), (1.0, 1.0)),
            Vertex::new((0.5, -0.5, 0.5), (1.0, 0.0)),
            Vertex::new((-0.5, -0.5, 0.5), (0.0, 0.0)),
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 1.0)),
            Vertex::new((0.5, 0.5, -0.5), (2.0, 1.0)),
            Vertex::new((0.5, -0.5, -0.5), (2.0, 0.0)),
            Vertex::new((-0.5, -0.5, -0.5), (3.0, 0.0)),
            Vertex::new((-0.5, 0.5, -0.5), (3.0, 1.0)),
        ];

        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();

        let indices = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &[
                0u8, 3, 1, 3, 2, 1, 0, 1, 5, 5, 4, 0, 4, 5, 6, 6, 7, 4, 7, 6, 2, 2, 3, 7, 4, 7, 3,
                3, 0, 4, 2, 6, 5, 5, 1, 2,
            ],
        )
        .unwrap();

        let texture = create_texture(display);

        Self {
            program,
            vertex_buffer,
            indices,
            texture,
            light_color: (1.0, 1.0, 1.0),
        }
    }

    fn draw_with_frame(&self, frame: &mut Frame, camera_mat: Mat4, dramparams: &DrawParameters) {
        let uniforms = uniform! {
            camera: [
                *camera_mat.as_array()[0].as_array(),
                *camera_mat.as_array()[1].as_array(),
                *camera_mat.as_array()[2].as_array(),
                *camera_mat.as_array()[3].as_array(),
            ],
            tnt_texture:
                Sampler::new(&self.texture)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
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

impl HairCube {
   pub fn set_light_color(&mut self, color: (f32, f32, f32)) {
        self.light_color = color;
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
}
implement_vertex!(Vertex, position location(0), tex_coord location(1));

impl Vertex {
    fn new((x, y, z): (f32, f32, f32), (tx, ty): (f32, f32)) -> Self {
        Self {
            position: [x, y, z],
            tex_coord: [tx, ty],
        }
    }
}

fn create_texture(display: &Display) -> SrgbTexture2d {
    let image = image::load(
        Cursor::new(include_bytes!("tnt_side.png")),
        ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let id = image.dimensions();
    let image = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), id);
    SrgbTexture2d::new(display, image).unwrap()
}
