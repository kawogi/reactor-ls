pub mod bindings;

#[derive(Copy, Clone)]
pub enum Action {
    Accelerate,
    Decelerate,
    StrafeLeft,
    StrafeRight,
    Ascend,
    Descend,

    YawLeft, // turn left
    YawRight, // turn right
    PitchUp,
    PitchDown,
    RollLeft,
    RollRight,
}