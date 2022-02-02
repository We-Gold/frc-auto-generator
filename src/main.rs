use macroquad::prelude::*;
mod constants;
mod mouse;
mod point_manager;
mod pose;
mod ui_manager;

fn check_point_control(point_manager: &mut point_manager::PointManager) {
    // TODO split one point into two interpolated ones
    // TODO add bezier points

    if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
        // Press space or click to add a point
        point_manager.add_point(&mouse::get_mouse_pose());
    } else if is_key_pressed(KeyCode::Z) {
        // Click z to remove the last point
        point_manager.remove_selected_point();
    } else if is_key_down(KeyCode::R) {
        // Hold R to rotate the pose to mouse position
        point_manager.rotate_selected_point(&mouse::get_mouse_pose());
    } else if is_key_down(KeyCode::E) {
        // Hold E to edit (move the pose to mouse position)
        point_manager.edit_selected_point(&mouse::get_mouse_pose());
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

#[macroquad::main("FRC Auto Generator")]
async fn main() {
    let mut point_manager = point_manager::PointManager::new();
    let mut ui_manager = ui_manager::UIManager::new();

    loop {
        clear_background(constants::BACKGROUND);

        // Draw the mouse information
        mouse::draw_mouse_position();

        // Update the GUI
        ui_manager.update_ui();

        // Add, remove, or clear points depending on the user input
        if ui_manager.clear_points_clicked() {
            point_manager.remove_all_points();
        } else if is_key_pressed(KeyCode::I) {
            // Toggle the GUI
            ui_manager.toggle_gui();
        } else if !ui_manager.output_text_selected() {
            check_point_control(&mut point_manager);
        }

        // Render the points and the lines connecting them
        point_manager.draw_all_points();

        // Update the json output text box
        ui_manager.set_output_text(point_manager.to_json());

        next_frame().await
    }
}
