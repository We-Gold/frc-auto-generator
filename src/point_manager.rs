use crate::constants::*;
use crate::pose::Pose;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use splines::{Interpolation, Key, Spline};

#[derive(Serialize, Deserialize)]
pub struct PointManager {
    spline: Spline<f32, Pose>,
    selected_point: Option<usize>,
}

impl PointManager {
    pub fn new() -> PointManager {
        return PointManager {
            spline: Spline::from_vec(Vec::new()),
            selected_point: None,
        };
    }

    pub fn add_point(&mut self, point: &Pose) {
        // Determine the index of the new point
        let index = self.spline.len();

        // Add the point to the spline
        self.spline
            .add(Key::new(index as f32, *point, Interpolation::Linear));

        // Update the selected point
        self.set_selected_point(Some(index));
    }

    pub fn set_selected_point(&mut self, index: Option<usize>) {
        self.selected_point = index;
    }

    pub fn get_selected_point_index(&self) -> usize {
        return self.selected_point.unwrap();
    }

    pub fn edit_selected_point(&mut self, point: &Pose) {
        self.get_selected_point_mut().update_position(point);
    }

    pub fn rotate_selected_point(&mut self, point: &Pose) {
        self.get_selected_point_mut().angle_towards_pose(point);
    }

    pub fn remove_selected_point(&mut self) {
        if self.selected_point != None {
            // Remove the point from the spline
            self.spline.remove(self.get_selected_point_index());

            // Set the selected point to the next least point
            self.update_selected_point_to_previous();
        }
    }

    pub fn update_selected_point_to_previous(&mut self) {
        if self.selected_point != None {
            // Set the selected point to the next least point
            self.set_selected_point(if self.get_selected_point_index() > 0 {
                Some(self.get_selected_point_index() - 1)
            } else if self.spline.len() == 0 {
                None
            } else {
                Some(self.spline.len() - 1)
            });
        }
    }

    pub fn update_selected_point_to_next(&mut self) {
        if self.selected_point != None {
            // Set the selected point to the next point
            self.set_selected_point(if self.spline.len() == 0 {
                None
            } else {
                Some((self.get_selected_point_index() + 1) % self.spline.len())
            });
        }
    }

    pub fn show_point_direction(point: &Pose) {
        // Create a line of set magnitude in the given direction
        let (end_x, end_y) = (
            point.x + POSE_DIRECTION_LENGTH * point.theta.cos(),
            point.y + POSE_DIRECTION_LENGTH * point.theta.sin(),
        );

        draw_line(point.x, point.y, end_x, end_y, 4., POSE_DIRECTION_COLOR);
    }

    fn draw_point(point: &Pose) {
        draw_circle(point.x, point.y, CIRCLE_RADIUS, CIRCLE_COLOR);
    }
    fn connect_points(point1: &Pose, point2: &Pose) {
        draw_line(point1.x, point1.y, point2.x, point2.y, 2., CIRCLE_COLOR)
    }
    fn show_selected_point(point: &Pose) {
        draw_circle(
            point.x,
            point.y,
            CIRCLE_RADIUS - CURSOR_THICKNESS,
            SELECTED_COLOR,
        );
    }

    pub fn draw_all_points(&self) {
        for (i, key) in self.spline.into_iter().enumerate() {
            if i + 1 < self.spline.len() {
                // Connect the given points
                PointManager::connect_points(&key.value, self.get_point(i + 1));
            }

            // Draw the angle of the point if it has one
            if key.value.theta != 0. {
                PointManager::show_point_direction(&key.value);
            }
            // Draw the point on the screen
            PointManager::draw_point(&key.value);

            if i == self.get_selected_point_index() {
                PointManager::show_selected_point(&key.value);
            }
        }
    }

    pub fn get_selected_point(&self) -> &Pose {
        return self.get_point(self.get_selected_point_index());
    }

    pub fn get_point(&self, index: usize) -> &Pose {
        return &self.spline.get(index).unwrap().value;
    }

    pub fn get_selected_point_mut(&mut self) -> &mut Pose {
        return self.get_point_mut(self.get_selected_point_index());
    }

    pub fn get_point_mut(&mut self, index: usize) -> &mut Pose {
        return self.spline.get_mut(index).unwrap().value;
    }

    pub fn remove_all_points(&mut self) {
        // Reset the state of the point manager
        self.spline = Spline::from_vec(Vec::new());
        self.selected_point = None;
    }

    pub fn to_json(&self) -> String {
        return to_string(&self).unwrap();
    }

    pub fn from_json(json: &str) -> PointManager {
        return from_str(json).unwrap();
    }
}
