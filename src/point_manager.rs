use crate::constants::*;
use crate::pose::Pose;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use splines::{Interpolation, Key, Spline};

#[derive(Serialize, Deserialize)]
pub struct PointManager {
    spline: Spline<f32, Pose>,
}

impl PointManager {
    pub fn new() -> PointManager {
        return PointManager {
            spline: Spline::from_vec(Vec::new()),
        };
    }

    pub fn add_point(&mut self, point: &Pose) {
        // Add the point to the spline
        self.spline.add(Key::new(
            self.spline.len() as f32,
            *point,
            Interpolation::Linear,
        ))
    }

    pub fn remove_last_point(&mut self) {
        let number_of_points = self.spline.len();

        if number_of_points > 0 {
            // Remove the point from the spline
            self.spline.remove(number_of_points - 1);
        }
    }

    fn draw_point(point: &Pose) {
        draw_circle(point.x, point.y, CIRCLE_RADIUS, CIRCLE_COLOR);
    }
    fn connect_points(point1: &Pose, point2: &Pose) {
        draw_line(point1.x, point1.y, point2.x, point2.y, 2., CIRCLE_COLOR)
    }

    pub fn draw_all_points(&self) {
        for (i, key) in self.spline.into_iter().enumerate() {
            // Draw the point on the screen
            PointManager::draw_point(&key.value);

            if i > 0 {
                // Connect the given points
                PointManager::connect_points(&key.value, &self.spline.get(i - 1).unwrap().value)
            }
        }
    }

    pub fn remove_all_points(&mut self) {
        self.spline = Spline::from_vec(Vec::new());
    }

    pub fn to_json(&self) -> String {
        return to_string(&self).unwrap();
    }

    pub fn from_json(json: &str) -> PointManager {
        return from_str(json).unwrap();
    }
}
