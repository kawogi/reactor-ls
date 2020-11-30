use cgmath::Rad;
use crate::input::bindings::Bindings;
use cgmath::{Matrix, Matrix3};
use cgmath::{Deg, InnerSpace, Matrix4, PerspectiveFov, Vector3, Vector4};
use glium::glutin;

#[allow(clippy::module_name_repetitions, clippy::clippy::struct_excessive_bools)]
pub struct CameraState {
    bindings: Bindings,
    aspect_ratio: f32,
    position: Vector3<f32>,

    matrix: Matrix3<f32>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    yaw_left: bool,
    yaw_right: bool,
    pitch_up: bool,
    pitch_down: bool,
    roll_left: bool,
    roll_right: bool,

}

impl CameraState {
    pub fn new(position: Vector3<f32>, look_at: Vector3<f32>, aspect_ratio: f32, bindings: Bindings) -> CameraState {

        // depth vector of the camery pointing into the observer's eye
        let cam_z = (position - look_at).normalize();
        // right vector of the camera
        let cam_x = Vector3::<f32>::unit_y().cross(cam_z).normalize();
        // up vector of the camera
        let cam_y = cam_z.cross(cam_x);

        CameraState {
            bindings,
            aspect_ratio,
            position,
            matrix: Matrix3::from_cols(cam_x, cam_y, cam_z),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,

            yaw_left: false,
            yaw_right: false,
            pitch_up: false,
            pitch_down: false,
            roll_left: false,
            roll_right: false,
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

    pub fn get_view(&self) -> Matrix4<f32> {
        let axis = self.matrix;

        let mut result = Matrix4::from(axis.transpose());
        // position of the world's origin in the camera's coordinate space
        result.w = Vector4::new(-self.position.dot(axis.x), -self.position.dot(axis.y), -self.position.dot(axis.z), 1.0);
        result
    }
    
    #[allow(clippy::cast_precision_loss)]
    pub fn update_position(&mut self) {

        let move_speed = 0.02;
        let rotate_speed = 0.02;

        let dy = if self.moving_up { 1 } else { 0 } - if self.moving_down { 1 } else { 0 };
        let dx = if self.moving_right { 1 } else { 0 } - if self.moving_left { 1 } else { 0 };
        let dz = if self.moving_backward { 1 } else { 0 } - if self.moving_forward { 1 } else { 0 };

        let yaw = if self.yaw_left { 1 } else { 0 } - if self.yaw_right { 1 } else { 0 };
        let pitch = if self.pitch_up { 1 } else { 0 } - if self.pitch_down { 1 } else { 0 };
        let roll = if self.roll_left { 1 } else { 0 } - if self.roll_right { 1 } else { 0 };

        let yaw = Matrix3::from_axis_angle(self.matrix.y, Rad(yaw as f32 * rotate_speed));
        let pitch = Matrix3::from_axis_angle(self.matrix.x, Rad(pitch as f32 * rotate_speed));
        let roll = Matrix3::from_axis_angle(self.matrix.z, Rad(roll as f32 * rotate_speed));

        // describes the movement of the camera in it's own coordinate system
        let half_movement = Vector3::new(dx as f32, dy as f32, dz as f32) * move_speed * 0.5;
        self.position += self.matrix * half_movement;

        self.matrix = roll * pitch * yaw * self.matrix;

        self.position += self.matrix * half_movement;
    }

    pub fn process_input(&mut self, event: &glutin::event::WindowEvent<'_>) {
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let pressed = input.state == glutin::event::ElementState::Pressed;
        if let Some(key) = input.virtual_keycode {
            if let Some(action) = self.bindings.map(key) {
                match action {
                    crate::input::Action::Accelerate => self.moving_forward = pressed,
                    crate::input::Action::Decelerate => self.moving_backward = pressed,
                    crate::input::Action::StrafeLeft => self.moving_left = pressed,
                    crate::input::Action::StrafeRight => self.moving_right = pressed,
                    crate::input::Action::Ascend => self.moving_up = pressed,
                    crate::input::Action::Descend => self.moving_down = pressed,
                    crate::input::Action::YawLeft => self.yaw_left = pressed,
                    crate::input::Action::YawRight => self.yaw_right = pressed,
                    crate::input::Action::PitchUp => self.pitch_up = pressed,
                    crate::input::Action::PitchDown => self.pitch_down = pressed,
                    crate::input::Action::RollLeft => self.roll_left = pressed,
                    crate::input::Action::RollRight => self.roll_right = pressed,
                }
            }
        }
    }
}
