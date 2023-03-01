use std::time::Duration;
use crate::camera::{Camera, CameraMovement};
use glium::glutin::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glium::glutin::window::{CursorGrabMode, Window};

pub struct CameraHandler {
    cursor_in: bool,
    cursor_grabed: bool,
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,
    lshift_pressed: bool,
    space_pressed: bool,
    mouse_delta: (f32, f32),
}

impl CameraHandler {
    pub fn new() -> Self {
        Self {
            cursor_in: false,
            cursor_grabed: false,
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            lshift_pressed: false,
            space_pressed: false,
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn handle_event<T: 'static>(&mut self, event: &Event<T>, window: &Window) {
        match event {
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CursorEntered {..} => {
                        self.cursor_in = true;
                    },
                    WindowEvent::CursorLeft {..} => {
                        self.cursor_in = false;
                    },
                    _ => { },
                }
            },
            Event::DeviceEvent {event, ..} => {
                match event {
                    DeviceEvent::Key(KeyboardInput {virtual_keycode: Some(VirtualKeyCode::W), state, ..}) => {
                        self.w_pressed = *state == ElementState::Pressed;
                    },
                    DeviceEvent::Key(KeyboardInput {virtual_keycode: Some(VirtualKeyCode::S), state, ..}) => {
                        self.s_pressed = *state == ElementState::Pressed;
                    },
                    DeviceEvent::Key(KeyboardInput {virtual_keycode: Some(VirtualKeyCode::A), state, ..}) => {
                        self.a_pressed = *state == ElementState::Pressed;
                    },
                    DeviceEvent::Key(KeyboardInput {virtual_keycode: Some(VirtualKeyCode::D), state, ..}) => {
                        self.d_pressed = *state == ElementState::Pressed;
                    },
                    DeviceEvent::Key(KeyboardInput {virtual_keycode: Some(VirtualKeyCode::LShift), state, ..}) => {
                        self.lshift_pressed = *state == ElementState::Pressed;
                    },
                    DeviceEvent::Key(KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Space), state, ..}) => {
                        self.space_pressed = *state == ElementState::Pressed;
                    },
                    DeviceEvent::Key(KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Escape), state, ..}) => {
                        if *state == ElementState::Pressed && self.cursor_grabed {
                            window.set_cursor_grab(CursorGrabMode::None).unwrap(); //TODO: handle error
                            window.set_cursor_visible(true);
                            self.cursor_grabed = false;
                        }
                    },
                    DeviceEvent::MouseMotion {delta: (x, y)} => {
                        self.mouse_delta = (*x as f32, *y as f32);
                    },
                    DeviceEvent::Button {button: 1, state} => {
                        if *state == ElementState::Pressed && self.cursor_in && ! self.cursor_grabed {
                            window.set_cursor_grab(CursorGrabMode::Confined).unwrap(); //TODO: handle error
                            window.set_cursor_visible(false);
                            self.cursor_grabed = true;
                        }
                    },
                    _ => {},
                }
            }
            _ => {},
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, delta_time: Duration) {
        if ! self.cursor_grabed {
            return;
        }
        if self.w_pressed {
            camera.update(delta_time, CameraMovement::Forward);
        }
        if self.s_pressed {
            camera.update(delta_time, CameraMovement::Backward);
        }
        if self.a_pressed {
            camera.update(delta_time, CameraMovement::Left);
        }
        if self.d_pressed {
            camera.update(delta_time, CameraMovement::Right);
        }
        if self.lshift_pressed {
            camera.update(delta_time, CameraMovement::Down);
        }
        if self.space_pressed {
            camera.update(delta_time, CameraMovement::Up);
        }
        if self.mouse_delta.0 != 0.0 || self.mouse_delta.1 != 0.0 {
            camera.update(delta_time, CameraMovement::Rotate(self.mouse_delta.0, self.mouse_delta.1));
            self.mouse_delta = (0.0, 0.0);
        }
    }
}