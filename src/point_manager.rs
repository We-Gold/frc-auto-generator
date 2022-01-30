use crate::constants::*;
use macroquad::prelude::*;
use splines::{Interpolation, Key, Spline};

pub fn create_point_manager() -> PointManager {
    PointManager {
        spline: Spline::from_vec(Vec::new()),
    }
}

// TODO create pose class and create a serialization and deserialization for it.

pub struct PointManager {
    spline: Spline<f32, Vec2>,
}

impl PointManager {
    pub fn add_point(&mut self, point: &Vec2) {
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

    fn draw_point(point: &Vec2) {
        draw_circle(point.x, point.y, CIRCLE_RADIUS, CIRCLE_COLOR);
    }
    fn connect_points(point1: &Vec2, point2: &Vec2) {
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

    // pub fn to_json(&self) {
    //     to_string(&self.spline);
    // }
}
