use std::io::Cursor;

use glium::{
    Display, DrawParameters, Frame, implement_vertex, IndexBuffer, Program, Surface, texture,
    uniform, VertexBuffer,
};
use glium::texture::SrgbTexture2d;
use glium::uniforms::Sampler;
use glm::{Mat4, Vec3};
use image::ImageFormat;
use num_traits::One;

use crate::Drawable;

pub struct HairCube {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: IndexBuffer<u8>,
    texture: SrgbTexture2d,
    shift_map: SrgbTexture2d,
    light_color: (f32, f32, f32),
    light_pos: Vec3,
    camera_pos: Vec3,
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
            // 前面
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 0.0, 1.0), (0.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((0.5, 0.5, 0.5), (0.0, 0.0, 1.0), (1.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (0.0, 0.0, 1.0), (1.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((-0.5, -0.5, 0.5), (0.0, 0.0, 1.0), (0.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            // 右面
            Vertex::new((0.5, 0.5, 0.5), (1.0, 0.0, 0.0), (0.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((0.5, 0.5, -0.5), (1.0, 0.0, 0.0), (1.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((0.5, -0.5, -0.5), (1.0, 0.0, 0.0), (1.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (1.0, 0.0, 0.0), (0.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            // 后面
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 0.0, -1.0), (1.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((0.5, 0.5, -0.5), (0.0, 0.0, -1.0), (0.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((0.5, -0.5, -0.5), (0.0, 0.0, -1.0), (0.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((-0.5, -0.5, -0.5), (0.0, 0.0, -1.0), (1.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            // 左面
            Vertex::new((-0.5, 0.5, -0.5), (-1.0, 0.0, 0.0), (0.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((-0.5, -0.5, -0.5), (-1.0, 0.0, 0.0), (0.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((-0.5, -0.5, 0.5), (-1.0, 0.0, 0.0), (1.0, 1.0 / 3.0), (0.0, 1.0, 0.0)),
            Vertex::new((-0.5, 0.5, 0.5), (-1.0, 0.0, 0.0), (1.0, 2.0 / 3.0), (0.0, 1.0, 0.0)),
            // 上面
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 1.0, 0.0), (0.0, 1.0), (0.0, 0.0, -1.0)),
            Vertex::new((0.5, 0.5, -0.5), (0.0, 1.0, 0.0), (1.0, 1.0), (0.0, 0.0, -1.0)),
            Vertex::new((0.5, 0.5, 0.5), (0.0, 1.0, 0.0), (1.0, 2.0 / 3.0), (0.0, 0.0, -1.0)),
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 1.0, 0.0), (0.0, 2.0 / 3.0), (0.0, 0.0, -1.0)),
            // 下面
            Vertex::new((-0.5, -0.5, -0.5), (0.0, -1.0, 0.0), (0.0, 0.0), (0.0, 0.0, -1.0)),
            Vertex::new((0.5, -0.5, -0.5), (0.0, -1.0, 0.0), (1.0, 0.0), (0.0, 0.0, -1.0)),
            Vertex::new((0.5, -0.5, 0.5), (0.0, -1.0, 0.0), (1.0, 1.0 / 3.0), (0.0, 0.0, -1.0)),
            Vertex::new((-0.5, -0.5, 0.5), (0.0, -1.0, 0.0), (0.0, 1.0 / 3.0), (0.0, 0.0, -1.0)),
        ];

        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();

        let indices = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &[
                0, 3, 2, 0, 2, 1, // 前面
                4, 7, 6, 4, 6, 5, // 右面
                9, 10, 11, 9, 11, 8, // 后面
                12, 13, 14, 12, 14, 15, // 左面
                16, 19, 18, 16, 18, 17, // 上面
                23, 20, 21, 23, 21, 22, // 下面
            ],
        )
        .unwrap();

        let texture = create_texture(display);
        let shift_map = create_shift_map(display);

        Self {
            program,
            vertex_buffer,
            indices,
            texture,
            shift_map,
            light_color: (1.0, 1.0, 1.0),
            light_pos: Vec3::one(),
            camera_pos: Vec3::one(),
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
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            shift_map:
                Sampler::new(&self.texture)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            light_color: [self.light_color.0, self.light_color.1, self.light_color.2],
            light_pos: *self.light_pos.as_array(),
            camera_pos: *self.camera_pos.as_array(),
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
    pub fn set_light_pos(&mut self, pos: Vec3) {
        self.light_pos = pos;
    }
    pub fn set_camera_pos(&mut self, pos: Vec3) {
        self.camera_pos = pos;
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coord: [f32; 2],
    binormal: [f32; 3],
}
implement_vertex!(Vertex, position location(0), normal location(1), tex_coord location(2), binormal location(3));

impl Vertex {
    fn new(
        (x, y, z): (f32, f32, f32),
        (nx, ny, nz): (f32, f32, f32),
        (tx, ty): (f32, f32),
        (bx, by, bz): (f32, f32, f32),
    ) -> Self {
        Self {
            position: [x, y, z],
            normal: [nx, ny, nz],
            tex_coord: [tx, ty],
            binormal: [bx, by, bz],
        }
    }
}

fn create_texture(display: &Display) -> SrgbTexture2d {
    let image = image::load(Cursor::new(include_bytes!("hair1024.png")), ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let id = image.dimensions();
    let image = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), id);
    SrgbTexture2d::new(display, image).unwrap()
}

fn create_shift_map(display: &Display) -> SrgbTexture2d { //TODO: 改为合适的材质类型
    let image = image::load(Cursor::new(include_bytes!("shift_map1024.png")), ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let id = image.dimensions();
    let image = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), id);
    SrgbTexture2d::new(display, image).unwrap()

}
