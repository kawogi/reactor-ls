use crate::input::keyboard::KeyFunction;
use std::collections::HashMap;

use glium::glutin::event::VirtualKeyCode;

pub struct Bindings {
    map: HashMap<VirtualKeyCode, KeyFunction>,
}

impl Default for Bindings {
    fn default() -> Self {
        let mut map = HashMap::new();

        // usual race controls
        map.insert(VirtualKeyCode::W, KeyFunction::Accelerate);
        map.insert(VirtualKeyCode::S, KeyFunction::Decelerate);
        map.insert(VirtualKeyCode::A, KeyFunction::YawLeft);
        map.insert(VirtualKeyCode::D, KeyFunction::YawRight);

        // Zero-G controls
        map.insert(VirtualKeyCode::Up, KeyFunction::PitchDown);
        map.insert(VirtualKeyCode::Down, KeyFunction::PitchUp);
        map.insert(VirtualKeyCode::Left, KeyFunction::StrafeLeft);
        map.insert(VirtualKeyCode::Right, KeyFunction::StrafeRight);
        map.insert(VirtualKeyCode::Q, KeyFunction::RollLeft);
        map.insert(VirtualKeyCode::E, KeyFunction::RollRight);

        // flight controls
        map.insert(VirtualKeyCode::PageUp, KeyFunction::Ascend);
        map.insert(VirtualKeyCode::PageDown, KeyFunction::Descend);

        Self { map }
    }
}

impl Bindings {

    pub fn map(&self, key: VirtualKeyCode) -> Option<KeyFunction> {
        self.map.get(&key).copied()
    }

}