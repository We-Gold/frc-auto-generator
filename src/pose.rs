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

    pub fn get_angle_to_pose(&self, pose: &Pose) -> f32 {
        let (x, y) = (pose.x - self.x, pose.y - self.y);

        return y.atan2(x);
    }

    pub fn angle_towards_pose(&mut self, pose: &Pose) {
        self.theta = self.get_angle_to_pose(pose);
    }
}
