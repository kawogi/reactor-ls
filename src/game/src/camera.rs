use crate::input::Control;
use cgmath::Rad;
use cgmath::{Matrix, Matrix3};
use cgmath::{Deg, InnerSpace, Matrix4, PerspectiveFov, Vector3, Vector4};

#[allow(clippy::module_name_repetitions, clippy::clippy::struct_excessive_bools)]
pub struct CameraState {
    aspect_ratio: f32,
    position: Vector3<f32>,

    matrix: Matrix3<f32>,
}

impl CameraState {
    pub fn new(position: Vector3<f32>, look_at: Vector3<f32>, aspect_ratio: f32) -> CameraState {

        // depth vector of the camery pointing into the observer's eye
        let cam_z = (position - look_at).normalize();
        // right vector of the camera
        let cam_x = Vector3::<f32>::unit_y().cross(cam_z).normalize();
        // up vector of the camera
        let cam_y = cam_z.cross(cam_x);

        CameraState {
            aspect_ratio,
            position,
            matrix: Matrix3::from_cols(cam_x, cam_y, cam_z),
        }
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
    }

    pub fn get_perspective(&self) -> Matrix4<f32> {
        PerspectiveFov {
            fovy: Deg(90.0).into(),
            aspect: self.aspect_ratio,
            near: 0.1,
            far: 1024.0,
        }.into()
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let mut result = Matrix4::from(self.matrix.transpose());
        // position of the world's origin in the camera's coordinate space
        result.w = Vector4::new(-self.position.dot(self.matrix.x), -self.position.dot(self.matrix.y), -self.position.dot(self.matrix.z), 1.0);
        result
    }

    pub fn move_by(&mut self, movement: Vector3<f32>) {
        self.position += self.matrix * movement;
    }
    
    pub fn yaw(&mut self, angle: Rad<f32>) {
        self.matrix = Matrix3::from_axis_angle(self.matrix.y, angle) * self.matrix;
    }
    
    pub fn pitch(&mut self, angle: Rad<f32>) {
        self.matrix = Matrix3::from_axis_angle(self.matrix.x, angle) * self.matrix;
    }
    
    pub fn roll(&mut self, angle: Rad<f32>) {
        self.matrix = Matrix3::from_axis_angle(self.matrix.z, angle) * self.matrix;
    }
    
    #[allow(clippy::cast_precision_loss)]
    pub fn update_position(&mut self, control: &Control) {

        let move_speed = 0.1;
        let rotate_speed = 0.1;

        let dy = control.ascend.value;
        let dx = control.strafe.value;
        let dz = -control.thrust.value;

        let yaw = control.yaw.value;
        let pitch = control.pitch.value;
        let roll = control.roll.value;

        // describes the movement of the camera in it's own coordinate system
        let half_movement = Vector3::new(dx as f32, dy as f32, dz as f32) * move_speed * 0.5;

        //perform a move/2→rotate→move/2 sequence which should be more precise than move→rotate or rotate→move
        self.move_by(half_movement);
        self.yaw(Rad(yaw as f32 * rotate_speed));
        self.pitch(Rad(pitch as f32 * rotate_speed));
        self.roll(Rad(roll as f32 * rotate_speed));
        self.move_by(half_movement);

        // repair unit vectors to compensate numeric instabilities after rotation
        self.matrix.x = self.matrix.y.cross(self.matrix.z).normalize();
        self.matrix.y = self.matrix.z.cross(self.matrix.x).normalize();
        self.matrix.z = self.matrix.x.cross(self.matrix.y).normalize();
    }

}
