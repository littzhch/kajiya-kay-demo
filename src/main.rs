use std::error::Error;

use glium::{Display, Surface};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::{Event, StartCause, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glm::Vec3;

use kajiya_kay_demo::camera::Camera;
use kajiya_kay_demo::camera_events::CameraHandler;
use kajiya_kay_demo::Drawable;
use kajiya_kay_demo::hair_cube::HairCube;
use kajiya_kay_demo::light_source::Light;
use kajiya_kay_demo::refresh_rate::RefreshRate;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_title("kajiya-kay demo")
        .with_inner_size(PhysicalSize::new(800, 600));

    let display = Display::new(wb, ContextBuilder::new().with_depth_buffer(24), &event_loop)?;

    let drawparams = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let mut camera = Camera::new();
    let mut rate = RefreshRate::new(61.0);
    let mut camera_handler = CameraHandler::new();

    let light_color = (1.0, 1.0, 1.0);
    let light_pos = Vec3::new(2.0, 0.9, -4.0);

    let mut light = Light::init(&display);
    let mut hair_cube = HairCube::init(&display);
    light.set_light_color(light_color);
    light.set_light_pos(light_pos);
    hair_cube.set_light_color(light_color);
    hair_cube.set_light_pos(light_pos);

    event_loop.run(move |event, _, controlflow| {
        camera_handler.handle_event(&event, display.gl_window().window());

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *controlflow = ControlFlow::Exit;
                return;
            }
            Event::WindowEvent { .. } => {
                return;
            }
            Event::NewEvents(cause) => {
                match cause {
                    StartCause::Init | StartCause::ResumeTimeReached { .. } => (), // go on
                    _ => return,
                }
            }
            _ => return,
        }

        *controlflow = ControlFlow::WaitUntil(rate.refresh_now());
        camera_handler.update_camera(&mut camera, rate.interval());
        hair_cube.set_camera_pos(camera.get_camera_pos());

        let camera_mat = camera.get_mat();

        let mut target = display.draw();
        target.clear_color_and_depth(
            (
                0.2 * light_color.0,
                0.2 * light_color.1,
                0.2 * light_color.2,
                1.0,
            ),
            1.0,
        );
        light.draw_with_frame(&mut target, camera_mat, &drawparams);
        hair_cube.draw_with_frame(&mut target, camera_mat, &drawparams);
        target.finish().unwrap();
    })
}
