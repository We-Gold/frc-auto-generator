use macroquad::prelude::*;
mod constants;
mod mouse;
mod point_manager;
mod pose;
mod ui_manager;

fn check_point_control(point_manager: &mut point_manager::PointManager) {
    if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
        point_manager.add_point(&mouse::get_mouse_pose());
    } else if is_key_pressed(KeyCode::Z) {
        point_manager.remove_last_point();
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
        } else if is_key_pressed(KeyCode::E) {
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
