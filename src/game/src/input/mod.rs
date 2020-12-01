pub mod bindings;
pub mod keyboard;

// #[derive(Copy, Clone)]
// pub enum Action {
//     Thrust(f32), // accelerate, move forward
//     Sidestep(f32), // strafe right
//     Ascend(f32), // move upwards

//     Yaw(f32), // turn left/right
//     Pitch(f32), // pitch up/down
//     Roll(f32), // roll left/right
// }

#[derive(Default)]
pub struct GradientValue {
    pub value: f32,
    set_value: f32,
    speed: f32,
}

impl GradientValue {

    pub fn new(speed: f32) -> Self {
        Self {
            value: 0.0,
            set_value: 0.0,
            speed,
        }
    }

    pub fn update(&mut self, time: f32) {
        if let Some(ord) = self.value.partial_cmp(&self.set_value) {
            match ord {
                std::cmp::Ordering::Less => {
                    // increase value to bring it closer to the set value
                    self.value = (self.value + self.speed * time).min(self.set_value);
                }
                std::cmp::Ordering::Equal => {
                    // nothing to do
                }
                std::cmp::Ordering::Greater => {
                    // decrease value to bring it closer to the set value
                    self.value = (self.value - self.speed * time).max(self.set_value);
                }
            }
        }
    }

}

pub struct Control {
    pub thrust: GradientValue,
    pub strafe: GradientValue,
    pub ascend: GradientValue,
    pub yaw: GradientValue,
    pub pitch: GradientValue,
    pub roll: GradientValue,
}

impl Control {

    pub fn new(gradient_speed: f32) -> Self {
        Self {
            thrust: GradientValue::new(gradient_speed),
            strafe: GradientValue::new(gradient_speed),
            ascend: GradientValue::new(gradient_speed),
            yaw: GradientValue::new(gradient_speed),
            pitch: GradientValue::new(gradient_speed),
            roll: GradientValue::new(gradient_speed),
        }
    }

    pub fn update(&mut self, time: f32) {
        self.thrust.update(time);
        self.strafe.update(time);
        self.ascend.update(time);
        self.yaw.update(time);
        self.pitch.update(time);
        self.roll.update(time);
    }

    pub fn set_thrust(&mut self, value: f32) {
        self.thrust.set_value = value.clamp(-1.0, 1.0);
    }

    pub fn set_strafe(&mut self, value: f32) {
        self.strafe.set_value = value.clamp(-1.0, 1.0);
    }

    pub fn set_ascend(&mut self, value: f32) {
        self.ascend.set_value = value.clamp(-1.0, 1.0);
    }

    pub fn set_yaw(&mut self, value: f32) {
        self.yaw.set_value = value.clamp(-1.0, 1.0);
    }

    pub fn set_pitch(&mut self, value: f32) {
        self.pitch.set_value = value.clamp(-1.0, 1.0);
    }

    pub fn set_roll(&mut self, value: f32) {
        self.roll.set_value = value.clamp(-1.0, 1.0);
    }

}