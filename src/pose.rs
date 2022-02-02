use serde::{Deserialize, Serialize};
use splines::impl_Interpolate;
use std::ops::{Add, Div, Mul, Sub};

impl_Interpolate!(f32, Pose, std::f32::consts::PI);

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

impl Add<Pose> for Pose {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            theta: 0.,
        }
    }
}

impl Add<f32> for Pose {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            theta: 0.,
        }
    }
}

impl Sub<Pose> for Pose {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            theta: 0.,
        }
    }
}

impl Sub<f32> for Pose {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            theta: 0.,
        }
    }
}

impl Mul<Pose> for Pose {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            theta: 0.,
        }
    }
}

impl Mul<f32> for Pose {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            theta: 0.,
        }
    }
}

impl Div<Pose> for Pose {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            theta: 0.,
        }
    }
}

impl Div<f32> for Pose {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            theta: 0.,
        }
    }
}
