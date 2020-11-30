use std::collections::HashMap;

use glium::glutin::event::VirtualKeyCode;

use super::Action;


pub struct Bindings {
    map: HashMap<VirtualKeyCode, Action>,
}

impl Default for Bindings {
    fn default() -> Self {
        let mut map = HashMap::new();

        // usual race controls
        map.insert(VirtualKeyCode::W, Action::Accelerate);
        map.insert(VirtualKeyCode::S, Action::Decelerate);
        map.insert(VirtualKeyCode::A, Action::YawLeft);
        map.insert(VirtualKeyCode::D, Action::YawRight);

        // Zero-G controls
        map.insert(VirtualKeyCode::Up, Action::PitchDown);
        map.insert(VirtualKeyCode::Down, Action::PitchUp);
        map.insert(VirtualKeyCode::Left, Action::StrafeLeft);
        map.insert(VirtualKeyCode::Right, Action::StrafeRight);
        map.insert(VirtualKeyCode::Q, Action::RollLeft);
        map.insert(VirtualKeyCode::E, Action::RollRight);

        // flight controls
        map.insert(VirtualKeyCode::PageUp, Action::Ascend);
        map.insert(VirtualKeyCode::PageDown, Action::Descend);

        Self { map }
    }
}

impl Bindings {

    pub fn map(&self, key: VirtualKeyCode) -> Option<Action> {
        self.map.get(&key).copied()
    }

}