use glium::glutin::event::{ElementState, KeyboardInput};

use crate::Bindings;

#[derive(Copy, Clone)]
pub enum KeyFunction {
    Accelerate,
    Decelerate,
    StrafeRight,
    StrafeLeft,
    Ascend,
    Descend,
    YawLeft,
    YawRight,
    PitchUp,
    PitchDown,
    RollLeft,
    RollRight,
}

#[derive(Copy, Clone, Default)]
struct Orientation{
    positive: i8,
    negative: i8,
}

impl Orientation {

    pub fn set_positive(&mut self, positive: bool) -> bool {
        let positive = positive as i8;
        let changed = positive != self.positive;
        self.positive = positive;
        changed
    }

    pub fn set_negative(&mut self, negative: bool) -> bool {
        let negative = negative as i8;
        let changed = negative != self.negative;
        self.negative = negative;
        changed
    }

    pub fn value(self) -> i8 {
        self.positive - self.negative
    }

}

impl From<Orientation> for f32 {
    fn from(orientation: Orientation) -> Self {
        f32::from(orientation.value())
    }
}

#[derive(Default)]
pub struct Control {
    bindings: Bindings,

    thrust: Orientation,
    strafe: Orientation,
    ascend: Orientation,

    yaw: Orientation,
    pitch: Orientation,
    roll: Orientation,
}

impl Control {

    pub fn process_keyboard_input(&mut self, input: KeyboardInput, control: &mut super::Control) {
        let pressed = input.state == ElementState::Pressed;
        if let Some(key) = input.virtual_keycode {
            if let Some(action) = self.bindings.map(key) {
                match action {
                    KeyFunction::Accelerate => if self.thrust.set_positive(pressed) { control.set_thrust(self.thrust.into()) },
                    KeyFunction::Decelerate => if self.thrust.set_negative(pressed) { control.set_thrust(self.thrust.into()) },
                    KeyFunction::StrafeRight => if self.strafe.set_positive(pressed) { control.set_strafe(self.strafe.into()) },
                    KeyFunction::StrafeLeft => if self.strafe.set_negative(pressed) { control.set_strafe(self.strafe.into()) },
                    KeyFunction::Ascend => if self.ascend.set_positive(pressed) { control.set_ascend(self.ascend.into()) },
                    KeyFunction::Descend => if self.ascend.set_negative(pressed) { control.set_ascend(self.ascend.into()) },
                    KeyFunction::YawLeft => if self.yaw.set_positive(pressed) { control.set_yaw(self.yaw.into()) },
                    KeyFunction::YawRight => if self.yaw.set_negative(pressed) { control.set_yaw(self.yaw.into()) },
                    KeyFunction::PitchUp => if self.pitch.set_positive(pressed) { control.set_pitch(self.pitch.into()) },
                    KeyFunction::PitchDown => if self.pitch.set_negative(pressed) { control.set_pitch(self.pitch.into()) },
                    KeyFunction::RollLeft => if self.roll.set_positive(pressed) { control.set_roll(self.roll.into()) },
                    KeyFunction::RollRight => if self.roll.set_negative(pressed) { control.set_roll(self.roll.into()) },
                }
            }
        }
    }

}
