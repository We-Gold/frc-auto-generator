use crate::mouse::get_mouse_pose;
use crate::point_manager;
use macroquad::prelude::*;

pub fn check_point_control(point_manager: &mut point_manager::PointManager) {
    if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
        // Press space or click to add a point
        point_manager.add_point(&get_mouse_pose());
    } else if is_key_pressed(KeyCode::Z) {
        // Click z to remove the last point
        point_manager.remove_selected_point();
    } else if is_key_down(KeyCode::R) {
        // Hold R to rotate the pose to mouse position
        point_manager.rotate_selected_point(&get_mouse_pose());
    } else if is_key_down(KeyCode::E) {
        // Hold E to edit (move the pose to mouse position)
        point_manager.edit_selected_point(&get_mouse_pose());
    } else if is_key_pressed(KeyCode::D) {
        // Select the next point
        point_manager.update_selected_point_to_next();
    } else if is_key_pressed(KeyCode::A) {
        // Select the previous point
        point_manager.update_selected_point_to_previous();
    } else if is_key_pressed(KeyCode::C) {
        // Split the current point into two
        point_manager.split_selected_point();
    }
}
