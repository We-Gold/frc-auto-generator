use crate::image_manager::FieldImageManager;
use crate::mouse::{draw_mouse_rotation, get_mouse_pose};
use crate::point_manager::PointManager;
use macroquad::prelude::*;

pub fn check_point_control(point_manager: &mut PointManager, image_manager: &FieldImageManager) {
    // Check to see if the mouse is within the field area on the screen
    let mouse_is_within_field = image_manager.is_point_in_field(&get_mouse_pose());

    if (is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space))
        && mouse_is_within_field
    {
        // Press space or click to add a point
        let pose = image_manager.to_field_pose(&get_mouse_pose());
        point_manager.add_point(&pose);
    } else if is_key_pressed(KeyCode::Z) {
        // Click z to remove the last point
        point_manager.remove_selected_point();
    } else if is_key_down(KeyCode::R) && mouse_is_within_field {
        // Hold R to rotate the pose to mouse position
        let pose = image_manager.to_field_pose(&get_mouse_pose());
        point_manager.rotate_selected_point(&pose);

        // Show the angle the point is facing
        draw_mouse_rotation(point_manager.get_selected_point().theta.to_degrees());
    } else if is_key_down(KeyCode::E) && mouse_is_within_field {
        // Hold E to edit (move the pose to mouse position)
        let pose = image_manager.to_field_pose(&get_mouse_pose());
        point_manager.edit_selected_point(&pose);
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
