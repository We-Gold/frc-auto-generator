use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub theta: f32, // radians
}

impl Pose {
    pub fn new(x: f32, y: f32) -> Pose {
        return Pose::new_with_theta(x, y, 0.);
    }

    pub fn new_with_theta(x: f32, y: f32, theta: f32) -> Pose {
        return Pose { x, y, theta };
    }

    pub fn update_position(&mut self, pose: &Pose) {
        self.x = pose.x;
        self.y = pose.y;
    }

    pub fn update_theta(&mut self, theta: f32) {
        self.theta = theta;
    }

    pub fn get_angle_to_pose(&self, pose: &Pose) -> f32 {
        let (x, y) = (pose.x - self.x, pose.y - self.y);

        return y.atan2(x);
    }

    pub fn angle_towards_pose(&mut self, pose: &Pose) {
        self.update_theta(self.get_angle_to_pose(pose));
    }
}
