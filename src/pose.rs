use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub theta: f32, // radians
}

impl Pose {
    pub fn new(x: f32, y: f32) -> Pose {
        return Pose { x, y, theta: 0. };
    }

    pub fn new_with_theta(x: f32, y: f32, theta: f32) -> Pose {
        return Pose { x, y, theta };
    }
}
