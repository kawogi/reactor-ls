use cgmath::{Matrix, Matrix3};
use cgmath::{Deg, InnerSpace, Matrix4, PerspectiveFov, Vector3, Vector4};
use glium::glutin;

#[allow(clippy::module_name_repetitions, clippy::clippy::struct_excessive_bools)]
pub struct CameraState {
    aspect_ratio: f32,
    position: Vector3<f32>,
    direction: Vector3<f32>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new(aspect_ratio: f32) -> CameraState {
        CameraState {
            aspect_ratio,
            position: Vector3::new(1.0, 1.0, 1.0),
            direction: Vector3::new(-1.0, -1.0, -1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
    }

    // pub fn set_position(&mut self, pos: (f32, f32, f32)) {
    //     self.position = pos;
    // }

    // pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
    //     self.direction = dir;
    // }

    pub fn get_perspective(&self) -> Matrix4<f32> {
        PerspectiveFov {
            fovy: Deg(90.0).into(),
            aspect: self.aspect_ratio,
            near: 0.1,
            far: 1024.0,
        }.into()
    }

    fn get_axis(&self) -> Matrix3<f32> {
        // depth vector of the camery pointing into the observer's eye
        let cam_z = -self.direction.normalize();
        // right vector of the camera
        let cam_x = Vector3::<f32>::unit_y().cross(cam_z).normalize();
        // up vector of the camera
        let cam_y = cam_z.cross(cam_x);
        Matrix3::from_cols(cam_x, cam_y, cam_z)
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let axis = self.get_axis();

        let mut result = Matrix4::from(axis.transpose());
        // position of the world's origin in the camera's coordinate space
        result.w = Vector4::new(-self.position.dot(axis.x), -self.position.dot(axis.y), -self.position.dot(axis.z), 1.0);
        result
    }

    pub fn update_position(&mut self) {
        let axis = self.get_axis();

        let dy = if self.moving_up { 0.01 } else { 0.0 } - if self.moving_down { 0.01 } else { 0.0 };
        let dx = if self.moving_right { 0.01 } else { 0.0 } - if self.moving_left { 0.01 } else { 0.0 };
        let dz = if self.moving_backward { 0.01 } else { 0.0 } - if self.moving_forward { 0.01 } else { 0.0 };

        self.position += axis * Vector3::new(dx, dy, dz);
    }

    pub fn process_input(&mut self, event: &glutin::event::WindowEvent<'_>) {
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let pressed = input.state == glutin::event::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            glutin::event::VirtualKeyCode::Up => self.moving_up = pressed,
            glutin::event::VirtualKeyCode::Down => self.moving_down = pressed,
            glutin::event::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::event::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::event::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::event::VirtualKeyCode::S => self.moving_backward = pressed,
            _ => (),
        };
    }
}
