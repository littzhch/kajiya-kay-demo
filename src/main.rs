use std::io::Cursor;
use std::error::Error;
use glium::{Display, implement_vertex, Program, Surface, texture, uniform, VertexBuffer};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::{Event, StartCause, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::texture::{SrgbTexture2d};
use image::ImageFormat;

use kajiya_kay_demo::camera::Camera;
use kajiya_kay_demo::refresh_rate::RefreshRate;
use kajiya_kay_demo::camera_events::CameraHandler;
use kajiya_kay_demo::light_source::Light;
use kajiya_kay_demo::Drawable;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_title("a window")
        .with_inner_size(PhysicalSize::new(800, 600));

    let display = Display::new(wb, ContextBuilder::new().with_depth_buffer(24), &event_loop)?;


    let shape = vec![
        Vertex::new((0.5, 0.5, 0.5), (1.0, 0.0, 0.0), (1.0, 1.0)),
        Vertex::new((0.5,-0.5, 0.5), (1.0, 0.0, 0.0), (1.0, 0.0)),
        Vertex::new((-0.5,-0.5,0.5), (1.0, 0.0, 0.0), (0.0, 0.0)),
        Vertex::new((-0.5,0.5, 0.5), (1.0, 0.0, 0.0), (0.0, 1.0)),

        Vertex::new((0.5, 0.5, -0.5), (1.0, 0.0, 0.0), (2.0, 1.0)),
        Vertex::new((0.5,-0.5, -0.5), (1.0, 0.0, 0.0), (2.0, 0.0)),
        Vertex::new((-0.5,-0.5,-0.5), (1.0, 0.0, 0.0), (3.0, 0.0)),
        Vertex::new((-0.5,0.5, -0.5), (1.0, 0.0, 0.0), (3.0, 1.0)),
    ];

    let vertex_buffer = VertexBuffer::new(&display, &shape)?;
    let indices =
        glium::index::IndexBuffer::new(&display,
                                       glium::index::PrimitiveType::TrianglesList,
                                       &[0u16, 3, 1, 3, 2, 1,
                                           0, 1, 5, 5, 4, 0,
                                           4, 5, 6, 6, 7, 4,
                                           7, 6, 2, 2, 3, 7,
                                           4, 7, 3, 3, 0, 4,
                                           2, 6, 5, 5, 1, 2,
                                       ])?;

    let program = Program::from_source(
        &display,
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC,
        None)?;

    let texture1 = create_texture1(&display);
    let texture2 = create_texture2(&display);

    let model_mat = get_model_mat();

    let drawparams = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };


    let mut camera = Camera::new();
    let mut rate = RefreshRate::new(120.0);
    let mut camera_handler = CameraHandler::new();

    let light = Light::init(&display);

    event_loop.run(move |event, _, controlflow| {
        camera_handler.handle_event(&event, display.gl_window().window());

        match event {
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                *controlflow = ControlFlow::Exit;
                return;
            }
            Event::WindowEvent {..} => {
                return;
            }
            Event::NewEvents(cause) => {
                match cause {
                    StartCause::Init | StartCause::ResumeTimeReached {..} =>  (), // go on
                    _ => return,
                }
            }
            _ => return,
        }

        *controlflow = ControlFlow::WaitUntil(rate.refresh_now());
        camera_handler.update_camera(&mut camera, rate.interval());

        let final_mat = camera.get_mat() * model_mat;

        let uniforms = uniform! {
            transform: [
                *final_mat.as_array()[0].as_array(),
                *final_mat.as_array()[1].as_array(),
                *final_mat.as_array()[2].as_array(),
                *final_mat.as_array()[3].as_array(),

            ],
            texture1: glium::uniforms::Sampler::new(&texture1)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            texture2: &texture2,
        };


        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.3, 0.3, 1.0),1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &drawparams).unwrap();
        light.draw_with_frame(&mut target, final_mat, &drawparams);
        target.finish().unwrap();
    })
}

fn create_texture1(display: &Display) -> SrgbTexture2d {
    let image = image::load(Cursor::new(include_bytes!("weeping_vines_plant.png")),
                            ImageFormat::Png).unwrap().to_rgba8();
    let id = image.dimensions();
    let image = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), id);
    SrgbTexture2d::new(display, image).unwrap()
}

fn create_texture2(display: &Display) -> SrgbTexture2d {
    let image = image::load(Cursor::new(include_bytes!("awesomeface.png")),
                            ImageFormat::Png).unwrap().to_rgb8();
    let id = image.dimensions();
    let image = texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), id);
    SrgbTexture2d::new(display, image).unwrap()
}

fn get_model_mat() -> glm::Mat4 {
    use num_traits::identities::One;
    let axis = glm::vec3(1.0, 0.0, 0.0);
    glm::ext::rotate(&glm::Mat4::one(),  0.0, axis)
}


#[derive(Clone, Copy)]
struct Vertex {
    position: [f32;3],
    color: [f32;3],
    tex_coord: [f32;2],
}
implement_vertex!(Vertex, position location(0), color location(1), tex_coord location(2));

impl Vertex {
    pub fn new((x, y, z): (f32, f32, f32), (r, g, b): (f32, f32, f32), (tx, ty): (f32, f32)) -> Self {
        Self {
            position: [x, y, z],
            color: [r, g, b],
            tex_coord: [tx, ty],
        }
    }
}


const VERTEX_SHADER_SRC: &str = r#"
#version 460 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 tex_coord;
out vec3 ourColor;
out vec2 texCoord;
uniform mat4 transform;
void main()
{
   gl_Position = transform * vec4(position.x, position.y, position.z, 1.0);
   ourColor = color;
   texCoord = tex_coord;
}"#;

const FRAGMENT_SHADER_SRC: &str = r#"
#version 460 core
out vec4 FragColor;
in vec3 ourColor;
in vec2 texCoord;
uniform sampler2D texture1;
uniform sampler2D texture2;
void main()
{
    FragColor = mix(texture(texture1, texCoord), texture(texture2, texCoord), 0.0);
}"#;




